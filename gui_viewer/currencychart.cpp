#include "currencychart.h"
#include "../currency_requester/currency_requester.h"


CurrencyChart::CurrencyChart() {
    this->setTitle("CurrencyChart");
    this->addAxis(x, Qt::AlignLeft);
    y->setRange(0, 100);
    this->addAxis(y, Qt::AlignBottom);
    this->addSeries(serverSeries);
    serverSeries->setName("Server Values");
    serverSeries->setPointsVisible();
    serverSeries->attachAxis(x);
    serverSeries->attachAxis(y);

    this->addSeries(dejkstraSeries);
    dejkstraSeries->setName("Dejkstra Values");
    dejkstraSeries->setPointsVisible();
    dejkstraSeries->attachAxis(x);
    dejkstraSeries->attachAxis(y);
}

void CurrencyChart::addPoints(Answer ca)
{
    auto p = lastX++;
    x->setMax(std::ceil(qMax(qMax(ca.server, ca.dejksra), x->max())));
    serverSeries->append(p, ca.server);
    dejkstraSeries->append(p, ca.dejksra);
}

void CurrencyChart::clear()
{
    lastX = 0;
    serverSeries->clear();
    dejkstraSeries->clear();
}
