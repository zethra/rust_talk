.PHONY: all watch pages

all: rust_presentation.pdf

pages: rust_presentation.pdf
	mkdir -p public
	cp rust_presentation.pdf public/

watch: rust_presentation.pdf
	bash -c "while inotifywait -e close_write rust_presentation.md; do make; done"

rust_presentation.pdf: rust_presentation.md *.png
	pandoc rust_presentation.md -t beamer -o rust_presentation.pdf