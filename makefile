DIRECTORY = mlcpp
GITUSERID = brianmatzelle
EXE = main
DATA = makeData

REPODIR = ../$(DIRECTORY)
TARFILE = $(DIRECTORY).tar

FLAGS = -Wall -Wextra -g
CC = g++
BIN = bin
OBJ = obj

all: $(BIN)/$(EXE)

$(BIN)/$(EXE): $(OBJ)/Main.o $(OBJ)/Net.o $(OBJ)/Neuron.o $(OBJ)/TrainingData.o
	$(CC) $(FLAGS) $(OBJ)/Main.o $(OBJ)/Net.o $(OBJ)/Neuron.o $(OBJ)/TrainingData.o -o $@

$(OBJ)/Main.o: Main.cpp Net.h TrainingData.h
	$(CC) $(FLAGS) -c Main.cpp -o $@

$(OBJ)/Net.o: Net.cpp Net.h Neuron.h
	$(CC) $(FLAGS) -c Net.cpp -o $@

$(OBJ)/Neuron.o: Neuron.cpp Neuron.h
	$(CC) $(FLAGS) -c Neuron.cpp -o $@

$(OBJ)/TrainingData.o: TrainingData.cpp
	$(CC) $(FLAGS) -c TrainingData.cpp -o $@

data: $(BIN)/$(DATA)

$(BIN)/$(DATA): $(OBJ)/MakeTrainingData.o
	$(CC) $(FLAGS) $(OBJ)/MakeTrainingData.o -o $@

$(OBJ)/MakeTrainingData.o: MakeTrainingData.cpp
	$(CC) $(FLAGS) -c MakeTrainingData.cpp -o $@

tar:	clean
	tar cvvf $(TARFILE) $(REPODIR)
	gzip $(TARFILE)

clean:
	rm -f $(OBJ)/*.o $(BIN)/$(EXE) *.tar.gz
