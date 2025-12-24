#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QChart>
#include <QMessageBox>
#include "trequester.h"
#include "currencychart.h"
#include "currencytable.h"

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private:
    Ui::MainWindow *ui;
    TRequester* tRequester;
    CurrencyChart* currencyChart;
    CurrencyTable* currencyTable;
};
#endif // MAINWINDOW_H
