
all:
	docker run --rm -v $(shell pwd):/data moss/xelatex make pdf

pdf:
	xelatex -jobname=maitreyakv-resume main.tex
	xelatex -jobname=maitreyakv-resume main.tex

clean:
	rm -f maitreyakv-resume.*

