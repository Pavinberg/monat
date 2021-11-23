LOCAL_HISTORY_FILE="./.monat/history"
HOME_HISTORY_FILE="${HOME}/.monat/history"

pattern='^,[0-9]+$'

result=()

check_bash_version() {
	if [ "${BASH_VERSINFO[0]}" -ge 4 && "${BASH_VERSINFO[1]}" -ge 2 ]; then
		satisfy_version=1
	fi
}

load_result() {
	while read -r line; do
		result+=($line)
	done < $1
}

_mn() {
	# -c means a command
	if [ "${COMP_WORDS[$((COMP_CWORD-1))]}" == "-c" ]; then
		COMPREPLY=($(compgen -c "${COMP_WORDS[$COMP_CWORD]}"))
		return
	fi

	# -d means dive into a directory so complete in the directory
	# store the result now
	path_prefix=''
	for (( i=1; i<$((COMP_CWORD-1)); i++ )); do
		if [ "${COMP_WORDS[$i]}" == '-d' ]; then
			path_prefix=${COMP_WORDS[$((i+1))]}
			break
		fi
	done
	
	if [ "${COMP_WORDS[$COMP_CWORD]}" == "," ]; then
		result=()
		if [ -e "$LOCAL_HISTORY_FILE" ]; then
			loc="[Local monat]"
			load_result "$LOCAL_HISTORY_FILE"
		elif [ -e "$HOME_HISTORY_FILE" ]; then
			loc="[Home]"
			load_result "$HOME_HISTORY_FILE"
		fi
		i=1
		if [ "${#result[@]}" -eq 1 ]; then
			COMPREPLY=("${result[0]}")
		else
			for r in "${result[@]}";do
			# echo "$i -- $r"
			COMPREPLY+=("$i -- $r")
			i=$((i+1))
		done
		fi
		
	elif [[ "${COMP_WORDS[$COMP_CWORD]}" =~ $pattern ]]; then
		result=()
		if [ -e "$LOCAL_HISTORY_FILE" ]; then
			load_result "$LOCAL_HISTORY_FILE"
		elif [ -e "$HOME_HISTORY_FILE" ]; then
			load_result "$HOME_HISTORY_FILE"
		fi
		
		idx=${COMP_WORDS[${COMP_CWORD}]#,}
		# if [ -z $satisfy_version ]; then
		# 	compopt -o nospace
		# fi
		COMPREPLY=("${result[$((idx-1))]}/")
	# else
	# 	COMPREPLY=($(compgen -f  -- "${COMP_WORDS[$COMP_CWORD]}"))
		# 	compopt +o nospace
	else
		COMPREPLY=($(compgen -W "$(ls $path_prefix)" "${COMP_WORDS[$COMP_CWORD]}"))
	fi
}

# if [  $satisfy_version ]; then
# 	complete  -o default -o bashdefault -F _monat_completions monat mn
# else
complete  -o default -o bashdefault -o nospace -F _mn mn
# fi

