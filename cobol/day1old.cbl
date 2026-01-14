
        IDENTIFICATION DIVISION.
        PROGRAM-ID. DAY1.
        AUTHOR. GOLD.

        DATA DIVISION.
        WORKING-STORAGE SECTION.
        01 WS-VAR.
            05 WS-D PIC 9(2)
            05 WS-T PIC X(20)
            05 WS-CAT PIC X(22)
        01 WS-I PIC 9(03) VALUE 0.
        01 WS-DIAL.
            03 WS-NUM  OCCURS 5 TIMES INDEXED BY WS-IDX.
                05 WS-NUM-TWO         PIC 9(02).
                05 WS-NUM-THREE       PIC X(10).

        PROCEDURE DIVISION.
            MOVE ALL '42fourtwo' TO WS-DIAL.
            DISPLAY "Before Init: " WS-DIAL.

            INITIALIZE WS-DIAL.
            DISPLAY "After Init: " WS-DIAL.

            MOVE "00zero" TO WS-NUM(1).
            MOVE "01one" TO WS-NUM(2).
            MOVE 3 TO WS-IDX.
            PERFORM 3 TIMES
                DISPLAY "Iteration: " WS-IDX
                MOVE WS-IDX TO WS-NUM(WS-IDX)
                COMPUTE WS-IDX = WS-IDX + 1
            END-PERFORM.

            MOVE "3three" to WS-NUM(3).
            DISPLAY "Everything in DIAL: " WS-DIAL.

            UNSTRING WS-NUM(1)
            DISPLAY "One: " WS-NUM(1).
            DISPLAY "Five: " WS-NUM(5).

            STOP RUN.
