#ifndef _TRAININGDATA_H_
#define _TRAININGDATA_H_
#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <sstream>

class TrainingData {
public:
    TrainingData(const std::string filename);
    bool isEof(void) { return m_trainingDataFile.eof(); }
    void getTopology(std::vector<unsigned> &topology);
    unsigned getNextInputs(std::vector<double> &inputVals);
    unsigned getTargetOutputs(std::vector<double> &targetOutpulVals);

private:
    std::ifstream m_trainingDataFile;
};

#endif