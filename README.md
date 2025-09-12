#Kahoot Finder#
Bot to find Kahoot Games. Because Kahoot has a public API, it is easy to check random pins until you find an active one.
Python is preferred, it is much improved over the rust one. 
# WARNING: USE ONLY FOR EDUCATIONAL PURPOSES. TEST ONLY IN AN CONTROLLED ENVIRONMENT. I AM NOT RESPONSIBLE FOR WHAT YOU DO WITH THIS SCRIPT. USING IT IN A NON EDUCATIONAL MANNER MAY VIOLATE KAHOOTS' EULA. DO NOT ABUSE THIS PLEASE! I RELEASE MYSELF FROM ALL RESPONSIBILITY REGARDING OTHER PEOPLE'S USAGE OF THIS SCRIPT!!!!!!
Python environment:

- First Time: Run `setup.sh`. If it gives you a permission denied error, run `sudo chmod 777 setup.sh` to make it executable.
If that says something about being in the wrong directory, you need to learn about the command line before using this.
DO NOT RUN runexe.sh. That runs the output of the rust script after running cargo build.

It will put you in a virtualenv with (KahootFinder) in front of your prompt. To leave it, type `deactivate`. Type `python3 v2python/main.py NICKNAME PINCOUNT`
- Run it again:
Run `source KahootFinder/bin/activate` then `python3 v2python/main.py NICKNAME PINCOUNT`




- V1 is in rust. use runexe.sh to run.