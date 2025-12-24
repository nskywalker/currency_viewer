#include "mainwindow.h"

#include <QApplication>
// #include "../currency_requester/currency_requester.h"

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    MainWindow w;
    w.show();
    return a.exec();
}
