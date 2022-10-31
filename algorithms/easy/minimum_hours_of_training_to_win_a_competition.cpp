class Solution {
public:
  int minNumberOfHours(int initialEnergy, int initialExperience, vector<int>& energy, vector<int>& experience) {
    int allEnergy = accumulate(energy.begin(), energy.end(), 0);
    int trainingEnergy = initialEnergy > allEnergy ? 0 : allEnergy - initialEnergy + 1;
    int trainingExp = 0;
    for (auto& ex : experience) {
      if (initialExperience <= ex) {
        int diff = ex - initialExperience + 1;
        initialExperience += diff;
        trainingExp += diff;
      }
      initialExperience += ex;
    }
    return trainingEnergy + trainingExp;
  }
};
