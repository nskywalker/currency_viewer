QT       += core gui charts concurrent

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++20

TARGET = currency_viewer

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    currencychart.cpp \
    currencytable.cpp \
    main.cpp \
    mainwindow.cpp \
    trequester.cpp

HEADERS += \
    currencychart.h \
    currencytable.h \
    mainwindow.h \
    trequester.h \
    utils.h

FORMS += \
    mainwindow.ui

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

CONFIG(debug, debug|release): LIBS += -L$$PWD/../currency_requester/target/debug/ -lt_requester
CONFIG(release, debug|release): LIBS += -L$$PWD/../currency_requester/target/release/ -lt_requester

INCLUDEPATH += $$PWD/../currency_requester
DEPENDPATH += $$PWD/../currency_requester
