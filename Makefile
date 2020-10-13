.PHONE: all clean

all:
	echo "This is rust study programs"

clean:
	find . -name target -type d | xargs rm -rf
