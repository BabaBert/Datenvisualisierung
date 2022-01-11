
DATA = "data/geo/ncdc-merged-sfc-mntp.txt"
FILENAME = "data/image/data.png"

if __name__ == "__main__":
    import os, re
    from PIL import Image

    real_path = os.path.realpath(DATA)

    image = Image.new(mode = "L", size=(72*12, 36*(2022-1880)+1))
    try:
        with open(real_path) as f:
            lines = f.readlines()
            line = 0

            #years
            for j in range(2022-1880):

                #months
                for m in range(12):

                    #latitude
                    for y in range(1, 37):
                        i_lines = y+(37*m)+((37*12)*j)  #start at 1 and at 37; 0 is the year
                        result = list(map(int, re.findall("[-\d]+", lines[i_lines])))
                        line += 1

                        #longitude
                        for x in range(len(result)):
                            i_img = ((72*m)+x, ((36*j)+(y-1)))

                            #None
                            if result[x] == -9999:
                                image.putpixel(i_img, (0))
                            else:
                                tmp = int(max(-127, result[x]/10)) + 128
                                image.putpixel(i_img, (tmp))
    except Exception as e:
        print(e)
    finally:
        image.save(FILENAME)