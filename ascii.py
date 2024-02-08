import PIL.Image
from ascii_magic import AsciiArt
my_art = AsciiArt.from_image("images/lol.png")
my_art.to_terminal(columns = 30)
'''def main():
	path = input("./images/1.png")
	try:
		image = PIL.Image.open(path)
	except:
		print("Failed to find image")
	image = resize(image)
	grayscale_image = to_grayscale(image)
	ascii_str = pixel_to_ascii(grayscale_image)
	image_width = grayscale_image.width
	ascii_str_len = len(ascii_str)
	ascii_img = ""
	for i in range(0, ascii_str_len, image_width):
		ascii_img += ascii_str[i:i + image_width] + "\n"
	with open ("ascii_image.txt", "w") as f:
		f.write(ascii_image);
ASCII_CHARS = ["%", "?", "*", "+", ";", ":", ",", "."]
def resize(image, new_width = 20):
	old_width, old_height = image.size
	new_height = new_width*old_height/old_width
	return image.resize((new_width, new_height))
def to_grayscale(image):
	return image.convert("L")
def pixel_to_ascii(image):
	pixels = image.getdata()
	ascii_str = "";
	for pixel in pixels:
		ascii_str += ASCII_CHARS[pixel]
	return ascii_str
main()
'''
