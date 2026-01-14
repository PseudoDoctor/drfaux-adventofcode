https://hub.docker.com/r/rawpair/gnucobol
docker.io/rawpair/gnucobol:trixie
distrobox enter cobol-dev
cd adventofcode
cobc -x hello.cbl

      * If I is 0, increment J
      * IF I is negative, add 100
      * IF I is above 99, subtract 100
      * IF nothing, set output to "no op"
