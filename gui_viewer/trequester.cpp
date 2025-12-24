#include "trequester.h"


TRequester::TRequester(QObject *parent)
    : QObject{parent}
{
}

TRequester::~TRequester()
{
}



QStringList TRequester::getCurrencies()
{
    auto vals = get_currencies();
    QStringList res;
    res.reserve(vals.size);
    for (auto i = 0ull; i < vals.size; ++i) {
        res << QString(vals.array[i]);
    }
    delete_string_array(vals);
    return res;
}

CurrencyAnswer TRequester::getCurrencyExchange(QString from, QString to)
{
    return CurrencyAnswer{0, 0, nullptr, nullptr};
    // return get_currency_exchange(&client, from.toUtf8().data(), to.toUtf8().data());
}

void TRequester::workWithCurrencies(QString from, QString to, Dates dates)
{
    isWorking.store(true, std::memory_order_release);
    auto channel = create_channel();
    auto _ = QtConcurrent::run(&TRequester::recievePoints, this, channel.reciever, false);
    get_currency_exchange(from.toStdString().c_str(), to.toStdString().c_str(), channel.sender, dates);
    emit setFinished();
}

void TRequester::workWithoutCurrencies(Dates dates)
{
    isWorking.store(true, std::memory_order_release);
    auto channel = create_channel();
    auto _ = QtConcurrent::run(&TRequester::recievePoints, this, channel.reciever, true);
    get_profitable_exchange(channel.sender, dates);
    emit setFinished();
}

void TRequester::recievePoints(RecieverAnswer channel, bool needCls)
{
    auto cls = needCls ? [](const char* str){delete_cstring(str);} : [](const char* str){Q_UNUSED(str);};
    while (isWorking.load(std::memory_order::acquire)) {
        auto points = read_from_reciever(channel);
        if (std::isnan(points.server_value) && std::isnan(points.dejkstra_value)) {
            break;
        }
        emit sendPoints(Answer{.server = points.server_value, .dejksra = points.dejkstra_value, .from = QString(points.from), .to = QString(points.to)});
        cls(points.from);
        cls(points.to);
    }
    drop_reciever(channel);
}

void TRequester::stop()
{
    isWorking.store(false, std::memory_order_release);
}
