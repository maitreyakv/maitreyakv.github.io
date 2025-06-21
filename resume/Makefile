
all:
	docker run --rm -v $(shell pwd):/data moss/xelatex make pdf

pdf:
	xelatex -jobname=maitreya-resume main.tex
	xelatex -jobname=maitreya-resume main.tex

clean:
	rm -f maitreya-resume.*

