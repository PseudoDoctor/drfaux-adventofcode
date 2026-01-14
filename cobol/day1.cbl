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
        01 INPUT-VARS.
           05 INPUT-STRING PIC X(10).
           05 DIR-CHAR PIC X.
           05 MOVE-STRING PIC X(9).
           05 MOVE-NUM PIC 9(5).
        01 DIAL-VARS.
           05 POS-COUNT PIC 9(4) OCCURS 100 TIMES INDEXED BY I.
           05 PAST-ZERO-COUNT PIC 9(5).
        01 CALC-VARS.
           05 DIAL-POS PIC 9(3) VALUE 50.
           05 TEMP-NUM PIC S9(7).
           05 QUO-NUM  PIC S9(5).
           05 REM-NUM  PIC S9(5).
           05 TEMP-STR PIC X(99).
           05 TEMP-NUM2 PIC S9(7).
        01 DISPLAY-VARS.
            05 DISP-VAL    PIC 99.
            05 FILLER   PIC X(6) VALUE " SEEN ".
            05 DISP-COUNT PIC ZZZ9.
            05 FILLER   PIC X(6) VALUE " TIME ".

        PROCEDURE DIVISION.
        MAIN-PARA.
            OPEN INPUT INPUT-FILE

            INITIALIZE DIAL-VARS

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
            MOVE RAW-INPUT-ROW TO INPUT-STRING.
            UNSTRING INPUT-STRING(1:1) INTO DIR-CHAR.
            UNSTRING INPUT-STRING(2:) INTO MOVE-STRING.
            MOVE FUNCTION NUMVAL(MOVE-STRING) TO MOVE-NUM.

           IF DIR-CHAR = 'R'
               COMPUTE TEMP-NUM = DIAL-POS + MOVE-NUM
           ELSE IF DIR-CHAR = 'L'
               COMPUTE TEMP-NUM = DIAL-POS - MOVE-NUM
           END-IF.

           DIVIDE 100
              INTO TEMP-NUM
              GIVING QUO-NUM
              REMAINDER REM-NUM
           END-DIVIDE.


           IF REM-NUM < 0
      * Use previous dial position to check if it was already 0
              IF DIAL-POS NOT = 0
                     ADD 1 TO PAST-ZERO-COUNT
                     IF QUO-NUM < 0
                            COMPUTE TEMP-NUM2 = QUO-NUM * -1
                            DISPLAY "temp " TEMP-NUM
                                    " quo " QUO-NUM
                                    " rem " REM-NUM
                                    " 0c " PAST-ZERO-COUNT
                                    " tmp2 " TEMP-NUM2
                            COMPUTE PAST-ZERO-COUNT =
                                    PAST-ZERO-COUNT + (QUO-NUM * -1)
                     END-IF
              END-IF
              COMPUTE DIAL-POS = REM-NUM + 100
              END-COMPUTE
           ELSE IF REM-NUM = 0
              ADD 1 TO PAST-ZERO-COUNT
                     IF QUO-NUM > 1
                     COMPUTE PAST-ZERO-COUNT =
                             PAST-ZERO-COUNT + QUO-NUM
                     ELSE IF QUO-NUM < 0
                     COMPUTE PAST-ZERO-COUNT =
                             PAST-ZERO-COUNT + (QUO-NUM * -1)
                     END-IF
              MOVE REM-NUM TO DIAL-POS
           ELSE
              IF QUO-NUM > 0
                     COMPUTE PAST-ZERO-COUNT =
                             PAST-ZERO-COUNT + QUO-NUM
              END-IF
              MOVE REM-NUM TO DIAL-POS
           END-IF.
           DISPLAY " LAND-ZERO: " POS-COUNT(1).
           ADD 1 TO POS-COUNT(DIAL-POS + 1).

           DISPLAY "DIR-CHAR: " DIR-CHAR
                 " MOVE-NUM: " MOVE-NUM
                 " DIAL-POS: " DIAL-POS
                 " TEMP-NUM: " TEMP-NUM
                 " QUO-NUM: " QUO-NUM
                 " REM-NUM: " REM-NUM
                 " PAST-ZERO: " PAST-ZERO-COUNT
                 " LAND-ZERO: " POS-COUNT(1).

        5000-DISPLAY1.
            PERFORM VARYING I FROM 1 BY 1 UNTIL I > 100
               IF POS-COUNT(I) > 0
                   MOVE I TO DISP-VAL
                   SUBTRACT 1 FROM DISP-VAL
                   MOVE POS-COUNT(I) TO DISP-COUNT
                   DISPLAY DISPLAY-VARS
               END-IF
           END-PERFORM.
           DISPLAY PAST-ZERO-COUNT.
