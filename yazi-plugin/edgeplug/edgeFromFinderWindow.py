import os
import sys
from PyQt5 import QtWidgets, QtGui
from pathlib import Path

app = QtWidgets.QApplication(sys.argv)
os.system('''/usr/bin/osascript -e 'tell app "Finder" to set frontmost of process "python" to true' ''')
fileNames, _ = QtWidgets.QFileDialog.getOpenFileNames(None, "QFileDialog.getOpenFileName()", sys.argv[1], "All Files (*);;Python Files (*.py)", )
parent = [str(Path(fileNames[0]).parent.absolute())]
app.exit()
os.system('''/usr/bin/osascript -e 'tell app "Finder" to set frontmost of process "iTerm2" to true' ''')
# print(["hi, "]+fileNames)
for index, i in enumerate(parent+fileNames):
    if index+1 <= len(fileNames):
        print(i + ", ", end="")
    else:
        print(i)
