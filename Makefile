MESSAGE = $1
config: 
	cd .git && git remote add github https://github.com/lino-smart/multiplayer-fps.git
	
push: 
	git add . && git commit -m "$(MESSAGE)" && git push origin && git push github 

merge:
	git checkout $(TO) && git merge $(FROM)

speed:
	bash assets.sh
	clear && cargo run --release