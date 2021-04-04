# Frontend

## Info
- The directory named 'AI Racing Game' here is the unity project for the frontend
- 'test' is a directory for code to simulate the server. Luca is attempting some rust code to test out the UDP receiver loop
- Ports for UDP are 59827 (clientside) and 59828 (serverside)
- The standard size of a buffer right now is 256, this is arbitary and can be changed as it likely will need to be.

## To do
- menu scene to enter login type details
	* make text boxes comfy on every screen :)
	* code to send script and user details to server
	* code to switch scenes once server responds
- Network code to receive messages from server and update car
	* position
	* rotation
	* speed
- Smoothing movement of car by predicting next position (not a priority)
- Code to generate track scenes from files/handmake each track
- Leaderboard
	* network code to get leaderboard
	* UI to display leaderboard
	* links to user profiles shown on leaderboard
- Profile page to display info (need to at least make sure this is all stored in first iterartion)
	* get user info from server
	* previous code
	* previous times
	* best leaderboard placement

## First Iteration
- no login, just take name
- submits name and script to server
- car just represented very basically (eg. percentage, line)
