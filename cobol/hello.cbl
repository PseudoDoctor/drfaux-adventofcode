      * run cd build; cobc -x ../hello-world.cbl to compile.
      * run ./build/hello-world to run the compile program.
				Identification Division.
		    Program-ID. sampleCOBOL.

			Storage Division.
			Working-Storage Section.
			    01 WS-2 PIC 9(2).
				01 WS-X PIC X(2).

		    Procedure Division.
		    Main-Paragraph.
						Display "Hello World!"
		        Stop Run.
