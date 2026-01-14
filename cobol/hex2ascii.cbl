       IDENTIFICATION DIVISION.
       PROGRAM-ID. HEX2ASCII.
       AUTHOR. GOLD.

       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  HEX-DIGITS         PIC X(16) VALUE "0123456789ABCDEF".
       01  INPUT-HEX-PAIR     PIC X(2)  VALUE "4B".
       01  CHAR-RESULT        PIC X.

       01  CALC-VARS.
           05  VAL-1          PIC 9(3).
           05  VAL-2          PIC 9(3).
           05  TOTAL-DEC      PIC 9(3).

       PROCEDURE DIVISION.
      *                          Find position of first digit (e.g., '4')
                  INSPECT HEX-DIGITS TALLYING VAL-1
                      FOR CHARACTERS BEFORE INITIAL INPUT-HEX-PAIR(1:1).

      *                          Find position of second digit (e.g., '1')
                  INSPECT HEX-DIGITS TALLYING VAL-2
                      FOR CHARACTERS BEFORE INITIAL INPUT-HEX-PAIR(2:1).

      *                          Standard Hex math: (Digit1 * 16) + Digit2
                  COMPUTE TOTAL-DEC = (VAL-1 * 16) + VAL-2.

      *                          Move decimal value to char (requires a binary field or pointer)
                  MOVE FUNCTION CHAR(TOTAL-DEC + 1) TO CHAR-RESULT.
                  DISPLAY CHAR-RESULT.
