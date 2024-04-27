alias list="tagger list"
alias purge="source <(tagger purge)"
alias rx="tagger echo"

tag() {
	source <(tagger tag $@)
}
move() {
	source <(tagger move $@)
}
