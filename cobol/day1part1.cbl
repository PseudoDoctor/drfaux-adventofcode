        IDENTIFICATION DIVISION.
        PROGRAM-ID. DAY1.
        AUTHOR. GOLD.

        ENVIRONMENT DIVISION.
        INPUT-OUTPUT SECTION.
        FILE-CONTROL.
           SELECT INPUT-FILE ASSIGN TO "biginput.txt"
           ORGANIZATION IS LINE SEQUENTIAL.

        DATA DIVISION.
        FILE SECTION.
        FD  INPUT-FILE.
        01  RAW-INPUT-ROW    PIC X(10).

        WORKING-STORAGE SECTION.
        01 FILE-STAT PIC X VALUE 'N'.
            88 END-OF-FILE VALUE 'Y'.
        01 THE-VARS.
            05 DIAL-POSITION PIC S9(3) VALUE 50.
            05 MOVE-COUNT PIC 9(5).
            05 NUM-TEMP  PIC S9(7).
            05 DIRECTION-CHAR   PIC X.
            05 DISP-TEMP PIC X(99).
            05 REM-TEMP PIC S9(3).
            05 STRING-TEMP PIC X(10).
            05 STR-TEMP PIC X(10).
        01 HISTOGRAM-TBL.
            05 POSITION-COUNT   PIC 9(4) OCCURS 100 TIMES INDEXED BY I.
        01 DISPLAY-VARS.
            05 DISP-VAL    PIC 99.
            05 FILLER   PIC X(6) VALUE " asdf ".
            05 DISP-COUNT PIC ZZZ9.
            05 FILLER   PIC X(6) VALUE " ghjk ".

        PROCEDURE DIVISION.
        MAIN-PARA.
            OPEN INPUT INPUT-FILE
            INITIALIZE HISTOGRAM-TBL

            PERFORM UNTIL END-OF-FILE
                READ INPUT-FILE
                    AT END MOVE 'Y' TO FILE-STAT
                    NOT AT END PERFORM 3000-PROCESS1
                END-READ
            END-PERFORM

            CLOSE INPUT-FILE
            PERFORM 5000-DISPLAY1

            STOP RUN.

        3000-PROCESS1.
            MOVE RAW-INPUT-ROW TO STRING-TEMP.
            UNSTRING STRING-TEMP(1:1) INTO DIRECTION-CHAR.
            UNSTRING STRING-TEMP(2:) INTO STR-TEMP.
            MOVE FUNCTION NUMVAL(STR-TEMP) TO MOVE-COUNT.

            STRING STRING-TEMP DELIMITED BY SIZE, SPACE,
                   DIRECTION-CHAR DELIMITED BY SIZE, SPACE,
                   STR-TEMP DELIMITED BY SIZE, SPACE,
                   MOVE-COUNT DELIMITED BY SIZE, SPACE,
                   DIAL-POSITION DELIMITED BY SIZE, SPACE,
                   NUM-TEMP DELIMITED BY SIZE, SPACE,
                INTO DISP-TEMP
                     ON OVERFLOW DISPLAY "ERROR"
                 NOT ON OVERFLOW DISPLAY "Before: " DISP-TEMP
            END-STRING.

            IF DIRECTION-CHAR = 'R'
                COMPUTE NUM-TEMP = DIAL-POSITION + MOVE-COUNT
            ELSE IF DIRECTION-CHAR = 'L'
                COMPUTE NUM-TEMP = DIAL-POSITION - MOVE-COUNT
            END-IF
             DISPLAY "POS: " DIAL-POSITION " move " DIRECTION-CHAR " "
                 MOVE-COUNT " ticks. ".
           COMPUTE DIAL-POSITION = FUNCTION
                REM(NUM-TEMP + 100000, 100).
             DISPLAY "Add 10000 to " NUM-TEMP
                 " and take remainder results in new POS: "
                 DIAL-POSITION.
             DISPLAY "Old Count: " POSITION-COUNT(DIAL-POSITION + 1).
            ADD 1 TO POSITION-COUNT (DIAL-POSITION + 1).
             DISPLAY "New Count: " POSITION-COUNT(DIAL-POSITION + 1).

        5000-DISPLAY1.
            PERFORM VARYING I FROM 1 BY 1 UNTIL I > 100
               IF POSITION-COUNT(I) > 0
                   MOVE I TO DISP-VAL
                   SUBTRACT 1 FROM DISP-VAL
                   MOVE POSITION-COUNT(I) TO DISP-COUNT
                   DISPLAY DISPLAY-VARS
               END-IF
           END-PERFORM.
