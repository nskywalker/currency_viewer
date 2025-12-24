#ifndef CURRENCYTABLE_H
#define CURRENCYTABLE_H

#include <QAbstractTableModel>
#include <QObject>
#include "utils.h"

class CurrencyTable : public QAbstractTableModel
{
    Q_OBJECT
public:
    explicit CurrencyTable(QObject *parent = nullptr);
    virtual int rowCount([[maybe_unused]] const QModelIndex &parent) const override;
    virtual int columnCount([[maybe_unused]] const QModelIndex &parent) const override;
    virtual QVariant data(const QModelIndex &index, int role) const override;
    virtual QVariant headerData(int section, Qt::Orientation orientation, int role) const override;
    virtual Qt::ItemFlags flags([[maybe_unused]] const QModelIndex &index) const override;
    void addValues(Answer c);
    void clear();
private:
    QList<Answer> vals;
};

#endif // CURRENCYTABLE_H
