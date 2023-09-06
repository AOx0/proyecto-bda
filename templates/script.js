function load_map_data() {
  let data = {}
  
  console.log('Fetching')
  fetch("/map_percent",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data)
    }
  )
    .then((response) => response.json())
    .then((json) => update_map_data(json));
}

function calculateMean(numbers) {
  if (numbers.length === 0) {
    return 0;
  }

  const sum = numbers.reduce((acc, num) => acc + num, 0);
  return sum / numbers.length;
}

function erf(y) {
  // save the sign of y
  var sign = (y >= 0) ? 1 : -1;
  y = Math.abs(y);

  var a1 =  0.254829592;
  var a2 = -0.284496736;
  var a3 =  1.421413741;
  var a4 = -1.453152027;
  var a5 =  1.061405429;
  var p  =  0.3275911;

  var t = 1.0/(1.0 + p*y);
  var z = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * Math.exp(-y * y);

  return sign * z; 
}

function calculateStandardDeviation(numbers) {
  if (numbers.length === 0) {
    return 0;
  }

  const mean = calculateMean(numbers);
  const squaredDifferences = numbers.map(num => Math.pow(num - mean, 2));
  const variance = squaredDifferences.reduce((acc, val) => acc + val, 0) / numbers.length;
  const stdDeviation = Math.sqrt(variance);

  return stdDeviation;
}

function calculateProbabilityLessThan(x, mean, stdDeviation) {
  // Calculate the z-score (standard score) for the given x
  const z = (x - mean) / stdDeviation;

  // Use the cumulative distribution function (CDF) of the standard normal distribution
  const probability = 0.5 * (1 + erf(z / Math.sqrt(2)));

  return probability;
}

function update_map_data(data) {
  console.log('Updating')
  let vals = data.valores.map((v) => v/data.total);

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i < vals.length; i++) {
    // There's no map zone for undefined areas and outside the city
    if (i+1 != 4 && i+1 != 8) {
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      let color = `rgba(255,0,0, ${color2})`
      document.getElementById(`1-${i+1}`).style.fill = color
    }
  }
}


