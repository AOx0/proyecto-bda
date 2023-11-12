const MAIN_COLOR = "rgb(17, 24, 39)";

const NOMBRES = [
  "Alvaro Obregon",
  "Azcapotzalco",
  "Benito Juarez",
  "CDMX",
  "Coyoacan",
  "Cuajimalpa de Morelos",
  "Cuauhtemoc",
  "Fuera de CDMX",
  "Gustavo A. Madero",
  "Iztacalco",
  "Iztapalapa",
  "La Magdalena Contreras",
  "Miguel Hidalgo",
  "Milpa Alta",
  "Tlahuac",
  "Tlalpan",
  "Venustiano Carranza",
  "Xochimilco",
];

const CATEGORIAS = [
  "Delito de bajo impacto",
  "Feminicidio",
  "Hecho no delictivo",
  "Homicidio doloso",
  "Lesiones dolosas por disparo de arma de fuego",
  "Plagio o secuestro",
  "Robo a casa habitación con violencia",
  "Robo a cuentahabiente saliendo del cajero con violencia",
  "Robo a negocio con violencia",
  "Robo a pasajero a bordo de microbus con y sin violencia",
  "Robo a pasajero a bordo de taxi con violencia",
  "Robo a pasajero a bordo del metro con y sin violencia",
  "Robo a repartidor con y sin violencia",
  "Robo a transeunte en vía pública con y sin violencia",
  "Robo a transportista con y sin violencia",
  "Robo de vehículo con y sin violencia",
  "Secuestro",
  "Violación",
];

function linear_regression(data) {
  let result = []

  // Se espera que data llegue como un vector [entero]
  let data_xy = data.map((e, i) => {
    return [i+1, e];
  });

  const my_regression = regression.linear(
    data_xy
  );

  result = my_regression.points.map(([x, y]) => {
    return y;    
  });

  return result;
}

function init_draw_pinned_chart(num, data, cfg) {

  const dat = {
    "alcaldias": cfg.pinned,
    "categorias": data["categorias"],
    "annio_inicio": data["annio_inicio"],
    "annio_final": data["annio_final"],
  };

  fetch("c_por_mes",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(dat)
    }
  )
    .then((response) => response.json())
    .then((json) => {
    const ctx = document.getElementById(`pinned-${num}`);

    var totales = json["valores"].reduce((acc, array) => acc.map((sum, i) => sum + array[i]), new Array(json["valores"][0].length).fill(0));
    var pr_totales = totales.map(function (curr) {
        return curr/json["valores"].length;
    });
    const regresion = linear_regression(pr_totales);

    let datasets = json["valores"].map((e, i) => {return {
      label: NOMBRES[cfg.pinned[i]-1],
      data: e,
      borderWidth: 1
    };});

    datasets.splice(0, 0, {
      label: "",
      data: regresion,
      borderWidth: 1,
      pointRadius: 0,
    });
    
    if( window.myBar === undefined) {
      window.myBar = new Chart(ctx, {
        type: 'line',
        data: {
          labels: json["meses"],
          datasets: datasets,
        },
        options: {
          scales: {
            y: {
              beginAtZero: true
            }
          }
        }
      });
    } else {
      window.myBar.data.datasets = datasets;
      window.myBar.data.labels = json["meses"];
      window.myBar.update();
    }    

    for (let i = 0; i < NOMBRES.length; i++) {
      // There's no map zone for undefined areas and outside the city
      if (i + 1 != 4 && i + 1 != 8) {
        document.getElementById(`${num}-${i + 1}`).style.fill = MAIN_COLOR
      }
    }

    if (cfg.colores_en_mapa) {
      cfg.pinned.forEach((e, i) => {
        document.getElementById(`${num}-${e}`).style.fill = window.myBar.data.datasets[i + 1].borderColor;
      });
    }

  });
}

function change_map_info(id, edo) {
  let name = document.getElementById(`${id}-name`);
  name.innerHTML = `<p hx-trigger="load" hx-get="/health">${NOMBRES[edo - 1]}</p>`
}

function load_map_data(data, cfg, chart_cfg) {
  let input_fini = document.getElementById(`afini-${cfg['num']}`);
  let input_init = document.getElementById(`ainit-${cfg['num']}`);

  if (input_fini != undefined && input_fini != undefined) {
    let err = false;
    
    if (data['annio_inicio'] < 2016 || data['annio_inicio'] > 2023) {
      input_init.classList.add("text-rose-300")
      err = true
    } else {
      input_init.classList.remove("text-rose-300")
    };
    
    if (data['annio_final'] < 2016 || data['annio_final'] > 2023) {
      input_fini.classList.add("text-rose-300")
      err = true
    } else {
      input_fini.classList.remove("text-rose-300")
    };

    if (err) return;
  }

  console.log('Fetching')
  console.log(JSON.stringify(data))

  update_map_data(cfg.num, { "total": 2000, "valores": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] });
  fetch(cfg.endpoint,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data)
    }
  )
    .then((response) => response.json())
    .then((json) => {
      update_map_data(cfg.num, json);
      if (!(chart_cfg === undefined)) {
        init_draw_pinned_chart(cfg.num, data, chart_cfg);
    }
    });
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

  var a1 = 0.254829592;
  var a2 = -0.284496736;
  var a3 = 1.421413741;
  var a4 = -1.453152027;
  var a5 = 1.061405429;
  var p = 0.3275911;

  var t = 1.0 / (1.0 + p * y);
  var z = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * Math.exp(-y * y);

  return sign * z;
}

function load_years_data(data, cfg) {
  console.log('Fetching years')
  console.log(JSON.stringify(data))

  for (let i = 0; i <= 7; i++) {
      document.getElementById(`anio-${i + 2016}`).style.opacity = 0.2;
  }
  fetch(cfg.endpoint,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data)
    }
  )
    .then((response) => response.json())
    .then((json) => {
      update_years_data(cfg.num, json);
    });
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
  let vals = data.valores.map((v) => v / data.total);

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i < vals.length; i++) {
    // There's no map zone for undefined areas and outside the city
    if (i + 1 != 4 && i + 1 != 8) {
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      document.getElementById(`${n}-${i + 1}`).style.fill = MAIN_COLOR
      document.getElementById(`${n}-${i + 1}`).style.fillOpacity = color2
    }
  }
}

function update_years_data(n, data) {
  console.log(`Updating ${n} with ${data.total} y ${data.valores}`)
  let vals = data.valores.map((v) => v / data.total);

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i <= 7; i++) {
    // There's no map zone for undefined areas and outside the city
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      console.log(`anio-${i + 2016} a ${color2}`)
      document.getElementById(`anio-${i + 2016}`).style.backgroundColor = MAIN_COLOR;
      document.getElementById(`anio-${i + 2016}`).style.opacity = color2 + 0.1;
  }
}


