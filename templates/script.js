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

function update_map_data(data) {
  console.log('Updating')
  let vals = data.valores.map((v) => v/data.total);

  for (let i = 0; i < vals.length; i++) {
    // There's no map zone for undefined areas and outside the city
    if (i+1 != 4 && i+1 != 8) {
      let color = `rgba(255,0,0, ${vals[i]})`
      console.log(color)
      document.getElementById(`1-${i+1}`).style.fill = color
    }
  }
}


