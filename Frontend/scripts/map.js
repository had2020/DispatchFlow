let user_latitude = 0;
let user_longitude = 0;

function getLocation() {
  if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition(success, error);
  } else {
    alert("Geolocation is not supported by this browser.");
  }
}

function success(position) {
  console.log(position.coords.latitude + ", " + position.coords.longitude);
  user_latitude = position.coords.latitude;
  user_longitude = position.coords.longitude;

  let map = L.map("map").setView([user_latitude, user_longitude], 17); // third parameter is zoom
  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    maxZoom: 19,
  }).addTo(map);
  var marker = L.marker([user_latitude, user_longitude]).addTo(map);
}

function error() {
  alert("Sorry, no position available.");
}

getLocation();

// report for duty
