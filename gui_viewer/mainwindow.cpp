#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    tRequester = new TRequester;
    currencyChart = new CurrencyChart;
    ui->currencyChartView->setChart(currencyChart);
    currencyTable = new CurrencyTable;
    ui->currencyTableView->setModel(currencyTable);
    auto tReqThread = new QThread(this);
    tReqThread->start();
    tRequester->moveToThread(tReqThread);
    auto curs = tRequester->getCurrencies();
    ui->fromCurrency->addItems(curs);
    ui->toCurrency->addItems(curs);
    connect(ui->startStopBtn, &QPushButton::clicked, tRequester, [this](bool clicked) {
        if (clicked) {
            emit tRequester->clearData();
            auto from = ui->fromCurrency->currentText();
            auto to = ui->toCurrency->currentText();
            if (from == to) {
                emit tRequester->showErrorWindow("Одинаковые валюты");
                emit tRequester->setFinished();
                return;
            }
            QMetaObject::invokeMethod(ui->profRateBtn, &QPushButton::setEnabled, Qt::QueuedConnection, false);
            QMetaObject::invokeMethod(ui->fromCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, false);
            QMetaObject::invokeMethod(ui->toCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, false);
            auto toDate = [](const QDate& date) {
                return Date {.year = date.year(), .month = static_cast<uint8_t>(date.month()), .day = static_cast<uint8_t>(date.day())};
            };
            Dates dates{.from = toDate(ui->from_date->selectedDate()), .to = toDate(ui->to_date->selectedDate())};
            auto _ = QtConcurrent::run(&TRequester::workWithCurrencies, tRequester, from, to, dates);
        } else {
            tRequester->stop();
            QMetaObject::invokeMethod(ui->profRateBtn, &QPushButton::setEnabled, Qt::QueuedConnection, true);
            QMetaObject::invokeMethod(ui->fromCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, true);
            QMetaObject::invokeMethod(ui->toCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, true);
        }
    });
    connect(tRequester, &TRequester::sendPoints, currencyChart, &CurrencyChart::addPoints);
    connect(tRequester, &TRequester::sendPoints, currencyTable, &CurrencyTable::addValues);
    connect(tRequester, &TRequester::setFinished, this, [this]{
        ui->startStopBtn->setChecked(false);
        ui->profRateBtn->setChecked(false);
        ui->startStopBtn->setEnabled(true);
        ui->profRateBtn->setEnabled(true);
        ui->fromCurrency->setEnabled(true);
        ui->toCurrency->setEnabled(true);
    });
    connect(tRequester, &TRequester::showErrorWindow, this, [this](QString text){
        QMessageBox::critical(nullptr, "Ошибка", text);
    });
    connect(tRequester, &TRequester::clearData, currencyChart, &CurrencyChart::clear);
    connect(tRequester, &TRequester::clearData, currencyTable, &CurrencyTable::clear);

    connect(ui->profRateBtn, &QPushButton::clicked, tRequester, [this](bool clicked){
        if (clicked) {
            emit tRequester->clearData();
            QMetaObject::invokeMethod(ui->startStopBtn, &QPushButton::setEnabled, Qt::QueuedConnection, false);
            QMetaObject::invokeMethod(ui->fromCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, false);
            QMetaObject::invokeMethod(ui->toCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, false);
            auto toDate = [](const QDate& date) {
                return Date {.year = date.year(), .month = static_cast<uint8_t>(date.month()), .day = static_cast<uint8_t>(date.day())};
            };
            Dates dates{.from = toDate(ui->from_date->selectedDate()), .to = toDate(ui->to_date->selectedDate())};
            auto _ = QtConcurrent::run(&TRequester::workWithoutCurrencies, tRequester, dates);
        } else {
            tRequester->stop();
            QMetaObject::invokeMethod(ui->startStopBtn, &QPushButton::setEnabled, Qt::QueuedConnection, true);
            QMetaObject::invokeMethod(ui->fromCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, true);
            QMetaObject::invokeMethod(ui->toCurrency, &QComboBox::setEnabled, Qt::QueuedConnection, true);
        }
    });
}

MainWindow::~MainWindow()
{
    delete ui;
    tRequester->stop();
    tRequester->thread()->quit();
    delete tRequester;
}




