PIXELS_PER_METER = 100.0

inches = float(input("Enter a value in inches: "))
meters = inches / 39.3701
pixels = meters * PIXELS_PER_METER
print(f"{inches} inches is {pixels} pixels. Half is {pixels / 2} pixels.")