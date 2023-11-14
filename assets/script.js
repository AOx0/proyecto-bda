const MAIN_COLOR = "rgb(17, 24, 39)";

const DIAS = [
  "Lunes",
  "Martes",
  "Miercoles",
  "Jueves",
  "Viernes",
  "Sabado",
  "Domingo"
]

const MESES = [
  "ENE",
  "FEB",
  "MAR",
  "ABR",
  "MAY",
  "JUN",
  "JUL",
  "AGO",
  "SEP",
  "OCT",
  "NOV",
  "DIC",
]

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

  if (cfg.pinned.length == 0) return;

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

function change_map_info(id, edo, valores) {
    let name = document.getElementById(`${id}-name`);
    let value = document.getElementById(`${id}-value`);
  if (!(edo === undefined)) {
    name.innerHTML = `<p>${NOMBRES[edo - 1]}</p>`
    value.innerHTML = `<p>Incidentes: ${new Intl.NumberFormat('en-US', {}).format(valores.valores[edo - 1])}</p>`
  } else {
    name.innerHTML = `<p>Total</p>`
    value.innerHTML = `<p>Incidentes: ${new Intl.NumberFormat('en-US', {}).format(valores.total)}</p>`
  }
}

function load_top_anio(data, cfg) {
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
      const ctx = document.getElementById(`pinned-${cfg.num}`);
 
      if( window.myBar2 === undefined) {
        window.myBar2 = new Chart(ctx, {
          type: 'bar',
          data: {
            labels: json["nombres"],
            datasets: [{
              label: `Top delitos ${data.annio}`,
              data: json["valores"],
              borderWidth: 1,
              pointRadius: 0,
              fill: false,
              backgroundColor: [
                'rgba(255, 99, 132, 0.2)',
                'rgba(255, 159, 64, 0.2)',
                'rgba(255, 205, 86, 0.2)',
                'rgba(75, 192, 192, 0.2)',
                'rgba(54, 162, 235, 0.2)',
                'rgba(153, 102, 255, 0.2)',
                'rgba(201, 203, 207, 0.2)',
                'rgba(255, 99, 132, 0.2)',
                'rgba(255, 159, 64, 0.2)',
                'rgba(255, 205, 86, 0.2)',
                'rgba(75, 192, 192, 0.2)',
                'rgba(54, 162, 235, 0.2)',
                'rgba(153, 102, 255, 0.2)',
                'rgba(201, 203, 207, 0.2)',
                'rgba(255, 99, 132, 0.2)',
              ],
              borderColor: [
                'rgb(255, 99, 132)',
                'rgb(255, 159, 64)',
                'rgb(255, 205, 86)',
                'rgb(75, 192, 192)',
                'rgb(54, 162, 235)',
                'rgb(153, 102, 255)',
                'rgb(201, 203, 207)',
                'rgb(255, 99, 132)',
                'rgb(255, 159, 64)',
                'rgb(255, 205, 86)',
                'rgb(75, 192, 192)',
                'rgb(54, 162, 235)',
                'rgb(153, 102, 255)',
                'rgb(201, 203, 207)',
                'rgb(255, 99, 132)',
              ],            
            }],
          },
          options: {
             indexAxis: 'y',
             scales: {
              y: {
                beginAtZero: true
              }
            },
            plugins: {
                title: { 
                  display: false 
                },
               legend: {
                  display: false
               }
            }
          }
        });
      } else {
        window.myBar2.data.datasets = [{
            label: `Top delitos ${data.annio}`,
            data: json["valores"],
            borderWidth: 1,
            pointRadius: 0,
            fill: false,
            backgroundColor: [
              'rgba(255, 99, 132, 0.2)',
              'rgba(255, 159, 64, 0.2)',
              'rgba(255, 205, 86, 0.2)',
              'rgba(75, 192, 192, 0.2)',
              'rgba(54, 162, 235, 0.2)',
              'rgba(153, 102, 255, 0.2)',
              'rgba(201, 203, 207, 0.2)',
              'rgba(255, 99, 132, 0.2)',
              'rgba(255, 159, 64, 0.2)',
              'rgba(255, 205, 86, 0.2)',
              'rgba(75, 192, 192, 0.2)',
              'rgba(54, 162, 235, 0.2)',
              'rgba(153, 102, 255, 0.2)',
              'rgba(201, 203, 207, 0.2)',
              'rgba(255, 99, 132, 0.2)',
            ],
            borderColor: [
              'rgb(255, 99, 132)',
              'rgb(255, 159, 64)',
              'rgb(255, 205, 86)',
              'rgb(75, 192, 192)',
              'rgb(54, 162, 235)',
              'rgb(153, 102, 255)',
              'rgb(201, 203, 207)',
              'rgb(255, 99, 132)',
              'rgb(255, 159, 64)',
              'rgb(255, 205, 86)',
              'rgb(75, 192, 192)',
              'rgb(54, 162, 235)',
              'rgb(153, 102, 255)',
              'rgb(201, 203, 207)',
              'rgb(255, 99, 132)',
            ],            
        }];
        window.myBar2.data.labels = json["nombres"];
        window.myBar2.update();
      }    
    });
}

function load_map_data(data, cfg, chart_cfg, valores) {
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

  if (!(chart_cfg === undefined)) {
      init_draw_pinned_chart(cfg.num, data, chart_cfg);
  }

  update_map_data(cfg.num, { "total": 2000, "valores": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] }, valores);
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
      update_map_data(cfg.num, json, valores);
      change_map_info(cfg.num, undefined, valores);      
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

function load_years_data(data, cfg, valores) {
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
      update_years_data(cfg.num, json, valores);
    });
}

function load_dias_data(data, cfg, valores) {
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

  console.log('Fetching dias')
  console.log(JSON.stringify(data))

  for (let i = 1; i <= 7; i++) {
      document.getElementById(`dia-${i}`).style.opacity = 0.2;
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
      update_dias_data(cfg.num, json, valores);
    });
}

function load_hours_data(data, cfg, valores) {
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

  console.log('Fetching hours')
  console.log(JSON.stringify(data))

  for (let i = 1; i <= 24; i++) {
      document.getElementById(`hora-${i}`).style.opacity = 0.2;
  }

  if (data['pinned'].length == 0) {
    return;
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
      update_hours_data(cfg.num, json, valores);
    });
}

function load_month_data(data, cfg, valores) {
  console.log('Fetching months')
  console.log(JSON.stringify(data))

  for (let i = 0; i <= 7; i++) {
    for (let j = 0; j < 12; j++) {
       document.getElementById(`anio-${i + 2016}-mes-${j + 1}`).style.opacity = 0.2;
    }
  }
  for (let i = 0; i <= 7; i++) {
    const temp_data = {
      anio: i + 2016,
      categorias: data['categorias']
    };

    fetch(cfg.endpoint,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(temp_data)
    }
    )
    .then((response) => response.json())
    .then((json) => {
      update_mes_data(cfg.num, json, valores);
    });
  }
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

function update_map_data(n, data, valores) {
  // console.log(`Updating ${n} with ${data.total}`)
  valores['valores'] = data.valores;
  valores['total'] = data.total;
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

function update_years_data(n, data, valores) {
  // console.log(`Updating ${n} with ${data.total} y ${data.valores}`)
  let vals = data.valores.map((v) => v / data.total);
  console.log(valores);
  valores['anios'] = data.valores;

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i < 8; i++) {
    // There's no map zone for undefined areas and outside the city
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      console.log(`anio-${i + 2016} a ${color2}`)
      document.getElementById(`anio-${i + 2016}`).style.backgroundColor = MAIN_COLOR;
      document.getElementById(`anio-${i + 2016}`).style.opacity = color2 + 0.1;
  }
}

function update_dias_data(n, data, valores) {
  // console.log(`Updating ${n} with ${data.total} y ${data.valores}`)
  let vals = data.valores.map((v) => v / data.total);
  console.log(valores);
  valores['dias'] = data.valores;

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i < 7; i++) {
    // There's no map zone for undefined areas and outside the city
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      document.getElementById(`dia-${i + 1}`).style.backgroundColor = MAIN_COLOR;
      document.getElementById(`dia-${i + 1}`).style.opacity = color2 + 0.1;
  }
}

function update_hours_data(n, data, valores) {
  // console.log(`Updating ${n} with ${data.total} y ${data.valores}`)
  let vals = data.valores.map((v) => v / data.total);
  console.log(valores);
  valores['horas'] = data.valores;

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i < 24; i++) {
    // There's no map zone for undefined areas and outside the city
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      document.getElementById(`hora-${i + 1}`).style.backgroundColor = MAIN_COLOR;
      document.getElementById(`hora-${i + 1}`).style.opacity = color2 + 0.1;
  }
}

function update_mes_data(n, data, valores) {
  // console.log(`Updating ${n} with ${data.total} y ${data.valores}`)
  let vals = data.valores.map((v) => v / data.total);
  console.log(valores);
  valores[`y${data.anio}`] = data.valores;

  let r = calculateStandardDeviation(vals);
  let r2 = calculateMean(vals);

  for (let i = 0; i < 12; i++) {
    // There's no map zone for undefined areas and outside the city
      let color2 = calculateProbabilityLessThan(vals[i], r2, r);
      console.log(`anio-${data.anio}-${i + 1} a ${color2}`)
      document.getElementById(`anio-${data.anio}-mes-${i + 1}`).style.backgroundColor = MAIN_COLOR;
      document.getElementById(`anio-${data.anio}-mes-${i + 1}`).style.opacity = color2 + 0.1;
  }
}


