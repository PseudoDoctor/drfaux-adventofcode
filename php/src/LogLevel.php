<?php

namespace Onine;
enum LogLevel: int
{
    case DEBUG = 1;
    case VERBOSE = 20;
    case INFO = 50;
    case QUIET = 75;
    case WARNING = 98;
    case ALWAYS = 99;
}
