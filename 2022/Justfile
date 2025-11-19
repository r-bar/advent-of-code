BASE_URL := "https://adventofcode.com/2022"
COOKIE := env_var("COOKIE")

# show this list of commands
default:
	just --list

# create a new day folder
template-day num: && (template-readme num) (download-input num)
	mkdir -p $(just day-dir {{num}})/src/bin
	sed -i "s/#\"$(just day-dir {{num}})\"/\"$(just day-dir {{num}})\"/" Cargo.toml
	touch $(just day-dir {{num}})/answers.txt $(just day-dir {{num}})/example.txt
	cp templates/Cargo.toml $(just day-dir {{num}})/Cargo.toml
	sed -i s/NAME/$(just day-dir {{num}})/ $(just day-dir {{num}})/Cargo.toml
	cp templates/python.py $(just day-dir {{num}})/d{{num}}p1.py
	cp templates/python.py $(just day-dir {{num}})/d{{num}}p2.py
	cp templates/rust.rs $(just day-dir {{num}})/src/bin/d{{num}}p1.rs
	cp templates/rust.rs $(just day-dir {{num}})/src/bin/d{{num}}p2.rs
	git add $(just day-dir {{num}})


# output the name of the day directory
# This is used to centralize naming and account for the lack of interpolation in ``
[private]
day-dir num:
	@echo "day$(printf '%02d' {{num}})"


[private]
tmpdir:
	mkdir -p tmp


[private]
download-prompt num: tmpdir
	curl {{BASE_URL}}/day/{{num}} -H "Cookie: {{COOKIE}}" --fail > tmp/day{{num}}.html \
		|| rm -f tmp/day{{num}}.html

# create a README.md in the day folder with the challenge text
template-readme num: (download-prompt num)
	hq 'h2:contains(Day)' text < tmp/day{{num}}.html \
		| sed 's/^---/\#/' \
		| sed 's/ ---$$//' \
		> "$(just day-dir {{num}})/README.md"
	echo '## Part One' >> "$(just day-dir {{num}})/README.md"
	hq .day-desc data < tmp/day{{num}}.html \
		| pandoc -f html -t gfm \
		| sed '/# --- Day/d' \
		| sed '/Part Two/s/ ---//g' \
		| tee -a "$(just day-dir {{num}})/README.md"


# download and save the input for the given day in input.txt
download-input num:
	curl {{BASE_URL}}/day/{{num}}/input -H "Cookie: {{COOKIE}}" --fail > "$(just day-dir {{num}})/input.txt" \
		|| rm -f "$(just day-dir {{num}})/input.txt"


[private]
test num:
	#!/usr/bin/env bash
	set -euxo pipefail
	pwd
	echo {{COOKIE}}


# remove temporary files
clean:
	rm -r tmp
