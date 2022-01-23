import matplotlib.pyplot as plt

filename = "exported_spectrum.csv"

# Import data as a list of numbers
with open(filename) as textFile:
    data = textFile.read().split()          # split based on spaces
    data = [float(point) for point in data] # convert strings to floats

frequency = [0] * int((len(data)/2))
value = [0] * int((len(data)/2))

for i in range(0, int((len(data)/2))):
    frequency[i] = data[i*2]
    value[i] = data[i*2 + 1]

# Plot as a time series plot
# print(frequency)
# print(value)

plt.plot(frequency, value)
plt.show()