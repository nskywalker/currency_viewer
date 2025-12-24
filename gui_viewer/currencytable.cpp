#include "currencytable.h"


CurrencyTable::CurrencyTable(QObject *parent)
    : QAbstractTableModel{parent}
{}

int CurrencyTable::rowCount(const QModelIndex &parent) const
{
    return vals.size();
}

int CurrencyTable::columnCount(const QModelIndex &parent) const
{
    return 4;
}

QVariant CurrencyTable::data(const QModelIndex &index, int role) const
{
    if (role != Qt::DisplayRole) {
        return QVariant{};
    }
    QVariant res;
    const auto& row_data = vals[index.row()];
    switch (index.column()) {
    case 0:
        res = QString("%1-%2").arg(row_data.from).arg(row_data.to);
        break;
    case 1:
        res = row_data.server;
        break;
    case 2:
        res = row_data.dejksra;
        break;
    case 3:
        res = (row_data.dejksra - row_data.server) / qMax(row_data.dejksra, row_data.server) * 100.f;
        break;
    default:
        break;
    }
    return res;
}

QVariant CurrencyTable::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (role != Qt::DisplayRole && orientation != Qt::Vertical) {
        return QVariant{};
    }
    QVariant res;
    switch (section) {
    case 0:
        res = "Currencies";
        break;
    case 1:
        res = "Server value";
        break;
    case 2:
        res = "Dejkstra value";
        break;
    case 3:
        res = "Difference in percent";
        break;
    default:
        break;
    }
    return res;
}

Qt::ItemFlags CurrencyTable::flags(const QModelIndex &index) const
{
    return Qt::ItemIsSelectable | Qt::ItemIsEnabled;
}

void CurrencyTable::addValues(Answer ca)
{
    auto c = vals.size();
    beginInsertRows(QModelIndex(), c + 1, c + 1);
    vals << ca;
    endInsertRows();
    // emit dataChanged(createIndex(c + 1, 0, nullptr), createIndex(c + 1, 0, nullptr));
}

void CurrencyTable::clear()
{
    beginRemoveRows(QModelIndex(), 0, vals.size() - 1);
    vals.clear();
    endRemoveRows();
}
