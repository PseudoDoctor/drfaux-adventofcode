
        IDENTIFICATION DIVISION.
        PROGRAM-ID. DAY1.
        AUTHOR. GOLD.

        DATA DIVISION.
        WORKING-STORAGE SECTION.
        01 WS-I PIC S99 VALUE 50.
        01 WS-J PIC ZZ VALUE 0.
        01 WS-OUTPUT PIC X(20).
        01 WS-INPUT.
            03 WS-LR PIC X VALUE "X".
            03 WS-C PIC X(2) VALUE "0".

        PROCEDURE DIVISION.
        MAIN-PARA.

            MOVE "L68" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "L30" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "R48" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "L5" TO WS-INPUT. FAILS ON SINGLE DIGIT.
            PERFORM 3000-PROCESS1.
            MOVE "R60" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "L55" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "L1" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "L99" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "R14" TO WS-INPUT.
            PERFORM 3000-PROCESS1.
            MOVE "L82" TO WS-INPUT.
            PERFORM 3000-PROCESS1.


            STOP RUN.
        1000-LEFT.
            SET WS-I DOWN BY WS-C.
        1000-RIGHT.
            SET WS-I UP BY WS-C.

        3000-PROCESS1.
            DISPLAY "I: " WS-I " LR:" WS-LR " C: " WS-C.
            IF WS-LR EQUAL "L"
                PERFORM 1000-LEFT
            ELSE
                IF WS-LR EQUAL "R"
                    PERFORM 1000-RIGHT
                END-IF
            END-IF.
            DISPLAY "I: " WS-I " J: " WS-J.
            MOVE "-" TO WS-OUTPUT.

            IF WS-I < 0
                MOVE "left of 0, loop it" TO WS-OUTPUT
                DISPLAY WS-OUTPUT
                SET WS-I UP BY 100
            END-IF.
            IF WS-I > 99
                MOVE "right of 0, loop it" TO WS-OUTPUT
                DISPLAY WS-OUTPUT
                SET WS-I DOWN BY 100
            END-IF.
            IF WS-I EQUAL 0
                MOVE "Incrementing J" TO WS-OUTPUT
                DISPLAY WS-OUTPUT
                SET WS-J UP BY 1
            END-IF.
            IF WS-OUTPUT EQUAL "-"
                DISPLAY "No Op"
            END-IF.

            DISPLAY "NEW I:" WS-I " J: " WS-J.
