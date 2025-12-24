use ndarray::{Array2, array};
use std::collections::{HashMap, LinkedList};

use crate::currency_getter::{self, CurrencyGetter};

#[derive(Debug, Clone)]
pub struct Edge {
    pub v1: String,
    pub v2: String,
    pub w: f32
}

#[derive(PartialEq, Debug)]
enum State {
    None,
    Seen,
}

impl Edge {
    pub fn create(v1_: String, v2_: String, w_: f32) -> Self {
        Edge { v1: v1_, v2: v2_ , w: w_}
    }
}

pub struct CurrencyGraph {
    matrix: ndarray::Array2<f32>,
    currency_to_number: HashMap<String, usize>,
    number_to_currency: HashMap<usize, String>
}

impl CurrencyGraph {
    fn new(m: ndarray::Array2<f32>, c_to_n: HashMap<String, usize>) -> Self {
        let n_to_c = c_to_n.iter().map(|(n, i)|(*i, n.clone())).collect();
        CurrencyGraph { matrix: m, currency_to_number: c_to_n, number_to_currency: n_to_c }
    }

    pub fn get_exchange(&self, from: String, to: String) -> Option<f32> {
        let v1 = match self.currency_to_number.get(&from) {
            Some(pv) => *pv,
            _ => return None
        };
        let v2 = match self.currency_to_number.get(&to) {
            Some(pv) => *pv,
            _ => return None
        };
        Some(self.matrix[[v1, v2]])
    }

    pub fn all_dejktstra(&self) -> Vec<Vec<Edge>> {
        self.all_dejktstra_impl(None).into_iter().map(|x|x.into_iter().collect()).collect()
    }

    fn all_dejktstra_impl(&self, null_v: Option<usize>) -> Vec<LinkedList<Edge>> {
        let mut ways = Vec::with_capacity(self.matrix.dim().0);
        for _ in 0..ways.capacity() {
            ways.push(f32::MAX);
        }
        let mut visited = Vec::with_capacity(self.matrix.dim().0);
        for _ in 0..visited.capacity() {
            visited.push(State::None);
        }
        if let Some(t) = ways.get_mut(null_v.unwrap_or(0)) {
            *t = 0_f32;
        } else {
            return Vec::new();
        }
        let mut chains = Vec::with_capacity(ways.capacity());
        for _ in 0..chains.capacity() {
            chains.push(LinkedList::<Edge>::new());
        }
        for _ in 0..self.matrix.dim().0 {
            let (min, min_w);
            if let Some((min_, min_w_)) = ways.iter().enumerate()
                    .filter(|&(i, _)| visited[i] == State::None).min_by(|(_, x), (_, y)| (**x).total_cmp(*y)) {
                        min = min_;
                        min_w = *min_w_;
            } else {
                break;
            }
            
            visited[min] = State::Seen;
            let filtered_ways = ways.iter_mut().enumerate()
                                    .filter(|&(i, _)|visited[i] == State::None)
                                    .filter(|&(i, _)| self.matrix.row(min)[i] != 0_f32);
            for (i, cur_way) in  filtered_ways {
                let weight = min_w + self.matrix.row(min)[i];
                if weight < *cur_way {
                    *cur_way = weight;
                    let chain_min = chains.get(min).unwrap().clone();
                    let chain_i = chains.get_mut(i).unwrap();
                    if !chain_min.is_empty() {
                        chain_i.clear();
                        for e in chain_min {
                            chain_i.push_back(e);
                        }
                    }
                    let e = Edge::create(self.number_to_currency.get(&min).unwrap().clone(),
                    self.number_to_currency.get(&i).unwrap().clone(), self.matrix[[min, i]]);
                    chain_i.push_back(e);
                }
            }
        }
        chains
    }

    pub fn dejkstra(&self, name: String, to_name: String) -> Vec<Edge> {
        let v = match self.currency_to_number.get(&name) {
            Some(pv) => *pv,
            _ => return Vec::new()
        };
        let v2 = match self.currency_to_number.get(&to_name) {
            Some(pv) => *pv,
            _ => return Vec::new()
        };
        let chains = self.all_dejktstra_impl(Some(v));
        chains[v2].iter().map(|e|e.clone()).collect()
    }

}

pub struct GraphBuilder;

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {}
    }

    pub fn build(&self) -> CurrencyGraph {
        let mut c_to_n = HashMap::new();
        c_to_n.insert("USD".to_string(), 0);
        c_to_n.insert("EUR".to_string(), 1);
        c_to_n.insert("CNY".to_string(), 2);
        c_to_n.insert("RUB".to_string(), 3);
        c_to_n.insert("TRY".to_string(), 4);
        let m = array![
            [0.0     , 0.8527  , 7.12    , 82.87, 41.57],
            [1.17    , 0.0     , 8.35    , 97.14, 48.75],
            [0.1404  , 0.1197  , 0.0     , 11.6 , 5.84],
            [0.012067, 0.010294, 0.086223, 0.0  , 0.5003],
            [0.024055, 0.020512, 0.1714  , 2.0    , 0.0]
        ];
        CurrencyGraph::new(m, c_to_n)
    }
    
}

pub struct CurrencyGraphBuilder {
    currencies: Vec<String>,
    date: time::Date
}

impl CurrencyGraphBuilder {
    pub fn new() -> Self {
        CurrencyGraphBuilder {currencies: Vec::new(), date: time::UtcDateTime::now().date()}
    }

    pub fn set_currencies(&mut self, c: Vec<String>) -> &mut Self {
        self.currencies = c;
        self
    }

    pub fn set_date(&mut self, date: time::Date) -> &mut Self {
        self.date = date;
        self
    }

    pub async fn build(&self) -> CurrencyGraph {
        // let rt = tokio::runtime::Builder::new_current_thread().enable_all().enable_io().build().unwrap();
        let mut m = Array2::<f32>::default((self.currencies.len(), self.currencies.len()));
        let c_to_n: HashMap<String, usize> = self.currencies.iter().enumerate().map(|(i, x)|(x.clone(), i)).collect();
        let mut hs = Vec::with_capacity(self.currencies.len());
        for c in self.currencies.iter() {
            let cc = c.clone();
            let date = self.date.clone();
            let handle = tokio::spawn(async move {
                let gc = CurrencyGetter::new();
                gc.currencies_at_date(&cc, date).await
            });
            hs.push(handle);
        } 
        for h in hs {
            let table = match h.await {
                Ok(opt_table) => match opt_table {
                    Some(_t) => _t,
                    _ => continue
                },
                _ => continue
            };
            let c_id = match c_to_n.get(table.base.as_str()) {
                Some(_c) => *_c,
                _ => continue
            };
            for (c2, val) in table.rates {
                let c2_id = match c_to_n.get(&c2) {
                    Some(_c) => *_c,
                    _ => continue
                };
                *m.get_mut((c_id, c2_id)).unwrap() = val;
            }
        }
        CurrencyGraph::new(m, c_to_n)
    }
    
}