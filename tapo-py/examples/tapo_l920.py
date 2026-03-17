

### A script based on the tapo API which reads a pre-populated .csv file ###
### containing dates and the colour of bin that needs to go out that day ###
###       and sets the light strip to the corresponding colours          ###

### I used task scheduler to have this run whenever I log in to my PC,   ###
###     but you could easily set up a raspberry pi to run this daily!    ###


# Install dependencies
import tapo
from tapo import ApiClient
from tapo.requests import (
    Color,
    LightingEffect,
    LightingEffectPreset,
    LightingEffectType,
)
import os
import asyncio
import datetime as dt
import pandas as pd

# Defines custom lighting effects for days with multiple bins
green_and_brown = (
        LightingEffect(
            name = "Green and Brown",
            type = LightingEffectType.Static,
            is_custom = True,
            enabled = True,
            brightness = 100,
            display_colors=[(0,0,0)],
        )
        .with_expansion_strategy(1)
        .with_segments([0, 1])
        .with_sequence([(125, 85, 66), (25, 85, 55)])
)

green_and_blue = (
        LightingEffect(
            name = "Green and Blue",
            type = LightingEffectType.Static,
            is_custom = True,
            enabled = True,
            brightness = 100,
            display_colors=[(0,0,0)],
        )
        .with_expansion_strategy(1)
        .with_segments([0, 1])
        .with_sequence([(125, 85, 66), (215, 80, 70)])
)


# Reads .csv using pandas; .csv has two columns, 'date' and 'bin_type'
today_date = pd.to_datetime(dt.date.today(), dayfirst=True)
bin_table = pd.read_csv('data.csv')

# Main loop
async def main():

    # Connect to client with username, password, and IP address
    client = ApiClient("<USERNAME>", "<PASSWORD>")
    device = await client.l920("<IP_ADDRESS>")

    # Check if today's date is in the 'date' column of the .csv
    # dayfirst = True to match the data in my .csv
    # Returns table containing the corresponding bin for today's date, if today is a bin day
    if today_date in pd.to_datetime(bin_table['date'], dayfirst=True).values:
        today_bin = bin_table[pd.to_datetime(bin_table['date'], dayfirst=True) == today_date]['bin_type'].values

        # if/elif blocks to check which bin/s need to go out and change the lights to corresponding colour
        # note that await device.on() is present in every block, to account for the lights being switched off between running code
        if "green" in today_bin and "brown" in today_bin:
            await device.on()
            await device.set_lighting_effect(green_and_brown)
        elif "green" in today_bin and "blue" in today_bin:
            await device.on()
            await device.set_lighting_effect(green_and_blue)
        elif "green" in today_bin:
            await device.on()
            await device.set_hue_saturation(125, 85)
        elif "brown" in today_bin:
            await device.on()
            await device.set_hue_saturation(25, 85)
        elif "blue" in today_bin:
            await device.on()
            await device.set_hue_saturation(215, 80)
        elif "brown" in today_bin:
            await device.on()
            await device.set_hue_saturation(125, 85)
        elif "purple" in today_bin:
            await device.on()
            await device.set_hue_saturation(270, 85)

    else:
        # If not a bin day, set lights to generic colour
        print("It's not bin day!")
        await device.on()
        await device.set_hue_saturation(195, 70)


if __name__ == "__main__":
    asyncio.run(main())

