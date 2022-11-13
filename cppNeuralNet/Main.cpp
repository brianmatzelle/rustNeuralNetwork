#include "Net.h"
#include "TrainingData.h"
#include <cassert>

void showVectorVals(std::string label, std::vector<double> &v) {
    std::cout << label << " ";
    for (unsigned i = 0; i < v.size(); ++i)
        std::cout << v[i] << " ";
    
    std::cout << std::endl;
}

int main() {
    TrainingData trainData("./data/trainingData.txt");
    std::vector<unsigned> topology;
    trainData.getTopology(topology);
    Net network(topology);

    std::vector<double> inputVals, targetVals, resultVals;
    int trainingPass = 0;
    
    while (!trainData.isEof()) {
        ++trainingPass;
        std::cout << "\n" << "Pass " << trainingPass;

        // get new input data and feed it forward
        if (trainData.getNextInputs(inputVals) != topology[0])
            break;
        showVectorVals(": Inputs:", inputVals);
        network.feedForward(inputVals);

        // collect the net's actual results
        network.getResults(resultVals);
        showVectorVals("Outputs:", resultVals);

        // train the net what the outputs should have been
        trainData.getTargetOutputs(targetVals);
        showVectorVals("Targets:", targetVals);
        assert(targetVals.size() == topology.back());

        // backpropagtion with target values
        network.backProp(targetVals);

        // report how well the training is working
        std::cout << "Net recent average error: "
            << network.getRecentAverageError() << "\n";
    }

    std::cout << std::endl << "Done" << std::endl;
}