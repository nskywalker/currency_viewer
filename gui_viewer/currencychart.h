#ifndef CURRENCYCHART_H
#define CURRENCYCHART_H

#include <QChart>
#include <QValueAxis>
#include <QLineSeries>
#include "utils.h"

class CurrencyChart : public QChart
{
    Q_OBJECT
public:
    CurrencyChart();
    void addPoints(Answer ca);
    void clear();
private:
    qreal lastX = 0;
    QValueAxis* x = new QValueAxis;
    QValueAxis* y = new QValueAxis;
    QLineSeries* serverSeries = new QLineSeries;
    QLineSeries* dejkstraSeries = new QLineSeries;
};

#endif // CURRENCYCHART_H
