#ifndef TREQUESTER_H
#define TREQUESTER_H

#include <QObject>
#include <atomic>
#include <QThread>
#include <QtConcurrent/QtConcurrent>
#include "../currency_requester/currency_requester.h"
#include "utils.h"

class TRequester : public QObject
{
    Q_OBJECT
public:
    explicit TRequester(QObject *parent = nullptr);
    ~TRequester();
    QStringList getCurrencies();
    CurrencyAnswer getCurrencyExchange(QString from, QString to);
    void workWithCurrencies(QString from, QString to, Dates dates);
    void workWithoutCurrencies(Dates dates);
    void recievePoints(RecieverAnswer channel, bool needCls);
    void stop();
signals:
    void sendPoints(Answer ca);
    void setFinished();
    void showErrorWindow(QString text);
    void clearData();
private:
    std::atomic<bool> isWorking{false};
};

#endif // TREQUESTER_H
