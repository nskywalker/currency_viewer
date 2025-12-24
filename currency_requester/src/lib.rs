use core::{f32, hash};
use std::{collections::{HashSet, HashMap}, ffi::{self, CStr, CString, c_char, c_uint, c_ulonglong, c_void}, fmt::{Pointer, format}, hash::Hasher, mem, ops::Div, ptr, slice::from_raw_parts, sync::{LazyLock, Mutex, atomic::AtomicPtr, mpsc::{self, Receiver, Sender}}, thread::{self, sleep}};
use time;

use crate::{currency_getter::CurrencyGetter, currency_graph::{CurrencyGraphBuilder, Edge}};
mod currency_graph;
mod currency_getter;
// use currency_graph;

#[repr(C)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

#[repr(C)]
pub struct Dates {
    from: Date,
    to: Date
}

struct DateRange {
    start: time::Date,
    end: time::Date
}

impl Iterator for DateRange {
    type Item = time::Date;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end {
            if let Some(next) = self.start.next_day() {
                self.start = next.clone();
                return Some(next);
            }
            return None;
        }
        None
    }
}

fn date_range(start: time::Date, end: time::Date) -> DateRange {
    DateRange { start: start, end: end }
}

#[repr(C)]
pub struct SenderAnswer {
    pub sender: *mut c_void
}

#[repr(C)]
pub struct RecieverAnswer {
    pub reciever: *mut c_void
}

#[repr(C)]
pub struct CurrencyAnswerChannel {
    pub sender: SenderAnswer,
    pub reciever: RecieverAnswer
}

#[no_mangle]
pub extern "C" fn create_channel() -> CurrencyAnswerChannel {
    let (tx, rx) = mpsc::channel::<CurrencyAnswer>();
    let btx = Box::new(tx);
    let brx = Box::new(rx);
    let sender = SenderAnswer {sender: Box::into_raw(btx) as *mut c_void};
    let reciever = RecieverAnswer {reciever: Box::into_raw(brx) as *mut c_void};
    CurrencyAnswerChannel { sender: sender, reciever: reciever }
}


#[repr(C)]
pub struct CurrencyAnswer {
    pub server_value: f32,
    pub dejkstra_value: f32,
    pub from: *const c_char,
    pub to: *const c_char
}

fn create_empty_currency_answer() -> CurrencyAnswer {
    CurrencyAnswer { server_value: f32::NAN, dejkstra_value: f32::NAN, from: ptr::null(), to: ptr::null() }
}

#[repr(C)]
pub struct StringArray {
    pub array: *mut *mut c_char,
    pub size: c_ulonglong
}

#[no_mangle]
pub unsafe extern "C" fn delete_string_array(str_arr: StringArray) {
    let t = from_raw_parts(str_arr.array, str_arr.size as usize);
    for tt in t {
        let _ = Box::from_raw(*tt);
    }
}

unsafe fn vec_of_cstring_to_raw(test: Vec<CString>) -> StringArray {
    let len = test.len();
    let box_test = test.into_boxed_slice();
    let v = (*box_test).iter().map(|s|s.clone().into_raw() as *mut c_char).collect::<Vec<*mut c_char>>();
    let aa = Box::into_raw(v.into_boxed_slice()) as *mut *mut i8;
    StringArray { array: aa, size: len as u64}
}

fn create_empty_string_array() -> StringArray {
    return StringArray { array: ptr::null_mut(), size: 0 }
}

#[no_mangle]
pub unsafe extern "C" fn get_currencies() -> StringArray {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let cg = CurrencyGetter::new();
    let res = rt.block_on(cg.currencies()).keys().map(|x|CString::new(x.clone()).unwrap()).collect();
    vec_of_cstring_to_raw(res)
}

#[no_mangle]
pub unsafe extern "C" fn drop_reciever(ch: RecieverAnswer) {
    let _ = ptr::read(ch.reciever as *mut Receiver<CurrencyAnswer>);
}

#[no_mangle]
pub unsafe extern "C" fn read_from_reciever(rcv: RecieverAnswer) -> CurrencyAnswer {
    if rcv.reciever.is_null() {
        return create_empty_currency_answer();
    }
    let ch_ptr = rcv.reciever as *mut Receiver<CurrencyAnswer>;
    if let Ok(answ) = ch_ptr.as_ref().unwrap().recv() {
        answ
    } else {
        create_empty_currency_answer()
    }
}

#[no_mangle]
pub extern "C" fn get_currency_exchange(from: *const c_char, to: *const c_char, ch: SenderAnswer, dates: Dates) {
    let tx = unsafe {ptr::read(ch.sender as *mut Sender<CurrencyAnswer>)};

    let from_id = match unsafe { CStr::from_ptr(from)  }.to_str() {
        Ok(id) => id.to_string(),
        _ => return
    };
    let to_id = match unsafe {CStr::from_ptr(to) }.to_str() {
        Ok(id) => id.to_string(),
        _ => return
    };

    let from_date = match time::Month::try_from(dates.from.month) {
        Ok(m) => match time::Date::from_calendar_date(dates.from.year, m, dates.from.day) {
            Ok(_from) => _from,
            _ => return
        },
        _ => return
    };
    let to_date = match time::Month::try_from(dates.to.month) {
        Ok(m) => match time::Date::from_calendar_date(dates.to.year, m, dates.to.day) {
            Ok(_to) => _to,
            _ => return
        },
        _ => return
    };

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let cg = CurrencyGetter::new();
    let currencies = rt.block_on(cg.currencies());
    let c_vec = currencies.into_iter().map(|x|x.0).collect::<Vec<_>>();
    for cur_date in date_range(from_date, to_date) {
        let g = rt.block_on(CurrencyGraphBuilder::new()
            .set_date(cur_date)
            .set_currencies(c_vec.clone())
            .build());
        let s = g.get_exchange(from_id.clone(), to_id.clone()).unwrap();
        let res = g.dejkstra(from_id.clone(), to_id.clone());
        let mut d = 1f32;
        res.into_iter().for_each(|x|d *= x.w);
        if tx.send(CurrencyAnswer { server_value: s, dejkstra_value: d, from: from, to: to }).is_err() {
            break;
        }
    }  
}

#[no_mangle]
pub unsafe extern "C" fn delete_cstring(str: *const c_char) {
    let _ = CString::from_raw(str as *mut c_char);
}

// #[no_mangle]
// pub unsafe extern "C" fn destroy_channel(ch: CurrencyAnswerChannel) {
//     let _ = Box::from_raw(ch.channel as *mut (Sender<CurrencyAnswer>, Receiver<CurrencyAnswer>));
// }

#[derive(PartialEq, Eq, Hash, Debug)]
struct FromToStruct {
    from: String,
    to: String
}

#[no_mangle]
pub extern "C" fn get_profitable_exchange(ch: SenderAnswer, dates: Dates) {
    let from_date = match time::Month::try_from(dates.from.month) {
        Ok(m) => match time::Date::from_calendar_date(dates.from.year, m, dates.from.day) {
            Ok(_from) => _from,
            _ => return
        },
        _ => return
    };
    let to_date = match time::Month::try_from(dates.to.month) {
        Ok(m) => match time::Date::from_calendar_date(dates.to.year, m, dates.to.day) {
            Ok(_to) => _to,
            _ => return
        },
        _ => return
    };

    let tx = unsafe {ptr::read(ch.sender as *mut Sender<CurrencyAnswer>)};

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let cg = CurrencyGetter::new();
    let currencies = rt.block_on(cg.currencies());
    let c_vec = currencies.into_iter().map(|x|x.0).collect::<Vec<_>>();
    for cur_date in date_range(from_date, to_date) {
        let g = rt.block_on(CurrencyGraphBuilder::new()
            .set_date(cur_date)
            .set_currencies(c_vec.clone())
            .build());
        let res = g.all_dejktstra();
        let mut answers = HashMap::new();
        for edges in res.iter().filter(|x|!x.is_empty()) {
            for j in 1..edges.len() {
                let front_edge = edges.get(j - 1).unwrap();
                let mut s = front_edge.w;
                let from = front_edge.v1.clone();
                for edge in edges.iter().skip(j) {
                    s *= edge.w;
                    let to = edge.v2.clone();
                    let cur = g.get_exchange(from.clone(), to.clone()).unwrap_or_default();
                    let greater = (s - cur) > 1e-5f32;
                    let e = FromToStruct {from: from.clone(), to: to.clone()};
                    if greater{
                        if !answers.contains_key(&e) {
                            answers.insert(e, (s, cur));
                        } else {
                            let (dejkstra, _) = answers.get_mut(&e).unwrap();
                            if *dejkstra - s > 1e-5f32 {
                                *dejkstra = s;
                            }
                        }
                        // answers.insert(CurrencyAnswer { server_value: cur, dejkstra_value: s, 
                        //     from: from_cstr, to: to_cstr });
                    }
                }
            }
        }
        for (e, (s, cur)) in answers.iter() {
            let a = CurrencyAnswer {dejkstra_value: s.clone(), server_value: cur.clone(), 
                from: CString::new(e.from.clone()).unwrap().into_raw() as *const c_char, 
                to: CString::new(e.to.clone()).unwrap().into_raw() as *const c_char};
            if tx.send(a).is_err() {
                break;
            }
        }
    }
}