import sys, os

dest = os.getcwd()
with open('/Users/edge/EdgeDocuments/EdgeGadgets/Apps/yazi/yazi-plugin/edgeplug/buffer', 'r') as f:
    files = f.read().splitlines() # get rid of the newline
    f.close()
open('/Users/edge/EdgeDocuments/EdgeGadgets/Apps/yazi/yazi-plugin/edgeplug/buffer', 'w').close()    # clear buffer

files = files[0][1:].split(" /")
files = ['/'+fi for fi in files]

if sys.argv[1] == "paste":
    for fi in files:
        newfi = dest + "/" + fi.split('/')[-1]
        os.popen("cp -r '{}' '{}'".format(fi, newfi))
elif sys.argv[1] == "link":
    for fi in files:
        newfi = dest + "/" + fi.split('/')[-1]
        if not os.path.exists(newfi):
            os.symlink(fi, newfi)
