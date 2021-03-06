const PI: f64 = 3.1415926535897932;

enum FilterType {
  Lowpass,
  Bandpass,
  Highpass
};

class autoWah {
public:
  autoWah();

  float runEffect(float x);

  void setFilterType(FilterType type);

  void setAttack(float tauA);
  void setRelease(float tauR);
  void setMinMaxFreq(float minFreq, float maxFreq);
  void setSampleRate(float fs);
  void setQualityFactor(float Q);
  void setMixing(float alphaMix);

private:
  float levelDetector(float x);
  float lowPassFilter(float x);
  float stateVariableFilter(float x);
  inline float mixer(float x, float y);

  float sin(float x);
  float precisionSin(float x);
  float tan(float x);
  float precisionTan(float x);

  // Sin and Tan Constants
  const float sinConst3, sinConst5;
  const float tanConst3, tanConst5;

  // Level Detector parameters
  float alphaA, alphaR, betaA, betaR;
  float bufferL[2];

  // Lowpass filter parameters
  float bufferLP, gainLP;

  // State Variable Filter parameters
  float minFreq, freqBandwidth;
  float q, fs, centerFreq;
  float yHighpass, yBandpass, yLowpass;
  float *yFilter;

  // Mixer parameters
  float alphaMix, betaMix;

};





autoWah::autoWah() : 
  yLowpass(0.0f), yBandpass(0.0f), yHighpass(0.0f), 
  yFilter(&yBandpass), fs(44.1e3), 
  sinConst3(-1.0f / 6.0f), sinConst5(1.0f / 120.0f), 
  tanConst3(1.0f / 3.0f), tanConst5(1.0f / 3.0f),
  bufferL(), bufferLP(0.0f)
{
  autoWah::setAttack(40e-3f);
  autoWah::setRelease(2e-3f);
  autoWah::setMinMaxFreq(20, 3000);
  autoWah::setQualityFactor(1.0f / 5.0f);
  autoWah::setMixing(1.0f);
}



float autoWah::runEffect(float x)
{
  float xL = x;
  if (xL < 0.0f) xL = -xL; // xL = abs(x)

  float yL = levelDetector(xL);

  //fc = yL * (maxFreq - minFreq) + minFreq;
  centerFreq = yL * freqBandwidth + minFreq;

  //float xF = x;
  float xF = lowPassFilter(x);
  float yF = stateVariableFilter(xF);

  float y = mixer(x, yF);

  return y;
}

void autoWah::setFilterType(FilterType type)
{
  switch(type) {
  case FilterType::Lowpass:
    yFilter = &yLowpass;
    break;
  case FilterType::Bandpass:
    yFilter = &yBandpass;
    break;
  case FilterType::Highpass:
    yFilter = &yHighpass;
    break;
  }
}

void autoWah::setAttack(float tauA)
{
  autoWah::alphaA = std::exp(-1.0 / (tauA*fs));
  autoWah::betaA = 1.0f - autoWah::alphaA;
}

void autoWah::setRelease(float tauR)
{
  autoWah::alphaR = std::exp(-1.0 / (tauR*fs));
  autoWah::betaR = 1.0f - autoWah::alphaR;
}

void autoWah::setMinMaxFreq(float minFreq, float maxFreq)
{
  autoWah::freqBandwidth = pi*(2.0f*maxFreq - minFreq)/fs;
  autoWah::minFreq = pi*minFreq/fs;
}

void autoWah::setSampleRate(float fs)
{
  autoWah::fs = fs;
}

void autoWah::setQualityFactor(float Q)
{
  autoWah::q = Q;
  autoWah::gainLP = std::sqrt(0.5f * q);
}

void autoWah::setMixing(float alphaMix)
{
  autoWah::alphaMix = alphaMix;
  autoWah::betaMix = 1.0f - alphaMix;
}

float autoWah::levelDetector(float x)
{
  float y1 = alphaR * bufferL[1] + betaR * x;
  if (x > y1) bufferL[1] = x;
  else          bufferL[1] = y1;

  bufferL[0] = alphaA * bufferL[0] + betaA * bufferL[1];

  return bufferL[0];
}

float autoWah::lowPassFilter(float x)
{
  //float K = std::tan(centerFreq);
  float K = autoWah::tan(centerFreq);
  float b0 = K / (K + 1);
  // b1 = b0;
  // a1 = (K - 1) / (K + 1);
  float a1 = 2.0f * (b0 - 0.5f);

  float xh = x - a1 * bufferLP;
  float y = b0 * (xh + bufferLP);
  bufferLP = xh;

  return gainLP * y;
}

float autoWah::stateVariableFilter(float x)
{
  float f = 2.0f * autoWah::sin(centerFreq);

  yHighpass  = x - yLowpass - q * yBandpass;
  yBandpass += f * yHighpass;
  yLowpass  += f * yBandpass;

  return *yFilter;
}

inline float autoWah::mixer(float x, float y)
{
  return alphaMix * y + betaMix * x;
}

float autoWah::sin(float x)
{
  return x * (1.0f + sinConst3*x*x);
}

float autoWah::precisionSin(float x)
{
  float x2 = x * x;
  float x4 = x2 * x2;
  return x * (1.0f + sinConst3*x2 + sinConst5*x4);
}

float autoWah::tan(float x)
{
  return x * (1.0f + tanConst3*x*x);
}

float autoWah::precisionTan(float x)
{
  float x2 = x * x;
  float x4 = x2 * x2;
  return x * (1.0f + tanConst3*x2 + tanConst5*x4);
}