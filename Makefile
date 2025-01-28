MESSAGE = $1
config: 
	cd .git && git remote add github https://github.com/lino-smart/multiplayer-fps.git
	
push: 
	git add . && git commit -m "$(MESSAGE)" && git push origin && git push github 

merge:
	git checkout $(TO) && git merge $(FROM)
download:
	bash assets.sh
speed:
	clear && cargo run --release