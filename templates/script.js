function change_map_info(id, edo) {
  let name = document.getElementById(`${id}-name`);
  name.innerHTML = `<p hx-trigger="load" hx-get="/health">${edo}</p>`
}

function load_map_data1(data) {
  console.log('Fetching')
  console.log(JSON.stringify(data))
  
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
    .then((json) => update_map_data(1, json));
}

function load_map_data2(data) {
  console.log('Fetching')
  console.log(JSON.stringify(data))
  
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
    .then((json) => update_map_data(2, json));
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

function update_map_data(n, data) {
  console.log(`Updating ${n} with ${data.total}`)
  let vals = data.valores.map((v) => v/data.total);

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i < vals.length; i++) {
    // There's no map zone for undefined areas and outside the city
    if (i+1 != 4 && i+1 != 8) {
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      let color = `rgba(17, 24, 39, ${color2})`
      document.getElementById(`${n}-${i+1}`).style.fill = color
    }
  }
}


