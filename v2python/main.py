from selenium import webdriver
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
import time
import requests
import random
# for args
import sys
# Options to run headless (no GUI window). Remove if you want to see it.
options = Options()
# options.headless = True  # Uncomment to run without a window

# Start Firefox WebDriver
driver = webdriver.Firefox(options=options)
# open kahoot in advance
driver.get("https://kahoot.it")
time.sleep(2)
# Replace with the game PIN
game_pin = 123456
NICKNAME = "lavachicken"
PIN_COUNT = 50

def setVarsFromArgs():
    if sys.argv[1] == "help":
        print("usage: python main.py NICKNAME PIN_COUNT")
        print("defaults to 50 pins with the name \"lavachicken\"")
        quit()
    # i love python constants <3
    NICKNAME = sys.argv[1]
    PIN_COUNT = sys.argv[2]

# Check if a element exists by its class.
# returns a boolean.
def checkIfElementExists(divclass):
    isElementReturned = len(driver.find_elements(By.CLASS_NAME, divclass))
    if isElementReturned > 0:
        return True
    if isElementReturned == 0:
        return False

for i in range(PIN_COUNT):
    game_pin = random.randint(100_000, 1_000_000)
    response = requests.get(f"https://kahoot.it/reserve/session/{game_pin}")
    print(response.status_code)
    if response.status_code == 200:
        print(f"GAME PIN FOUND: PIN {game_pin} IS VALID!!!!!")
        time.sleep(0.5)
        # this enters game pin automatically
        pin_input = driver.find_element(By.NAME, "gameId")
        pin_input.send_keys(game_pin)
        pin_input.send_keys(Keys.RETURN)
        print("Entering Pin....")
        #try entering nickname here
        time.sleep(3)
        name_input = driver.find_element(By.NAME, 'nickname')
        name_input.send_keys(NICKNAME)
        name_input.send_keys(Keys.RETURN)
        print("Entering Name....")
        # if the error bar pops up keep going
        if not checkIfElementExists("notification-bar__Text-sc-1e4wbj0-1 jkdbXz"):
            print("No luck, game is dead.")
            continue
        else:
            if driver.title == "You're In!":
                print(f"Congrats! Pin {game_pin} worked!")
                # iykyk
                print("So long and thanks for all the fish. Quitting.")
                quit()

    else:
        print(f"pin {game_pin} is not valid")

print("No games found. Quitting.")
driver.quit()
quit()
