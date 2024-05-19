# FileNewer
FileNewer is my attempt at making alternative to windows default file explorer, using C++ and ImGUI.

The project's key aims are to address my main gripes with the default file explorer:
- [ ] Indexing speed
- [ ] Search speed
- [ ] Navigation speed
- [ ] PowerShell integration

*Feel free to suggest more improvments to add*

# Installation

**TODO**

# Building

When building the application please understand that at the moment using the Intel oneAPI DPC++/C++ Compiler with Ninja is merely a suggestion as no part of the application depends on it, it will be the only platform I will work to support. If others wish to add alternative support be it with forks or code updates that will be accepted with open arms.

When building the application make sure that you have loaded the oneAPI in the current terminal config, *by default this is installed to `C:\Program Files (x86)\Intel\oneAPI\setvars.bat`* and that you ensure the application is being built using Ninja and icx `cmake -G Ninja -DCMAKE_CXX_COMPILER=icx ..`