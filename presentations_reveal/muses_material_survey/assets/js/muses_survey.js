
/**
 * @module muses_servey.js
 * @author Benedict R. Gaster
 * @copyright University of West of England, 2019
 * 
 * web-socket API for talking with muses survey backend.
 * 
 * Most of the complex state logic is in the backend, here we are simply 
 * the view for the user.
 * 
 * See README.md for more details on the protocol between view/server.
 *
 * @license: See LICENCE
 */

//------------------------------------------------------------------------------

var ws;

const likertSlideNumber = 2;
const pressSlideNumber  = 3;
const sliderSlideNumber = 8;

// Globals for slider animation
var box_x = 20.0;
var user_x = 20.0;
var box_size = 150.0;

(function(window) {
    /**
     * Check that browser supports Web socket API, so far tested on
     *   Chrome and Firefox
     */
    if ("WebSocket" in window) {
	     console.log("Initializing Muses Survey Plugin...")

       window.requestAnimationFrame(updateSlider);

	      // open up remote server
        ws = new WebSocket('ws://localhost:8080');

	      ws.onopen = function() {
            // Web Socket is connected, send data using send()
            
            // send connected message to confirm start of protocol.
            sendMS({type: "connected"});
        }

        /**
         * handle messages from Muses survey server
         */
        ws.onmessage = function (evt) {
            console.log(evt.data);

            var data = JSON.parse(evt.data);
            if (data.type == "press") {
              // draw press circle and ring
              updatePressCircle(data.circle, data.ring);
            }
            else if (data.type == "slider") {
              // draw slider canvas
              user_x = data.user_x;
              box_x  = data.box_x;
              box_size = data.box_size;

              //updateSlider(data.user_x, data.box_x);
            }
            else if (data.type == "consentID") {
              document.getElementById("consent-id").innerHTML = data.id;
            }
            else if (data.type == "materialIndex") {
                // set current material index
                if (data.slide == likertSlideNumber) {
                  document.getElementById("materialIndex").innerHTML = data.value;
                }
                else if (data.slide == pressSlideNumber) {
                  document.getElementById("materialIndexPress").innerHTML = data.value;
                }
                else if (data.slide == sliderSlideNumber) {
                  document.getElementById("materialIndexSlider").innerHTML = data.value;
                }
            }
            else if (data.type == "gestureType") {
              // set current gesture type
              document.getElementById("gestureType").innerHTML = data.value;
            }
            else if (data.type == "goto") {
              // switch slide
              Reveal.slide( data.slide );
            }
            
        }

        ws.onclose = function() {
            // websocket is closed.
        }
    }
})(window);

// send message to server, we assume it can be converted to JSON
function sendMS(data) {
  ws.send(JSON.stringify(data));
}


//------------------------------------------------------------------------------
// UTILS
//------------------------------------------------------------------------------

Reveal.addEventListener( 'slidechanged', function( event ) {
  //ws.send(JSON.stringify({type: "slidechange", value: Reveal.getState().indexh}));
} );

//------------------------------------------------------------------------------
// LIKERTS
//------------------------------------------------------------------------------

// handle submission of 1 or more likert scales from a given slide
function submitRadios() {
  for (var i=0; i < arguments.length; i++) {
    submitLikert(arguments[i]);
  }
}

function submitLikert(likert) { 
  var ele = document.getElementsByName(likert); 
  for(i = 0; i < ele.length; i++) { 
    if(ele[i].checked) {
      sendMS({type: "likert", name: likert, value: parseInt(ele[i].value, 10)});
    }
  } 
}

//------------------------------------------------------------------------------
// Send begin message
//------------------------------------------------------------------------------

function sendBegin() {
  sendMS({"type": "begin"});
}

//------------------------------------------------------------------------------
// Consent
//------------------------------------------------------------------------------

function sendConsent() {
  sendMS({"type": "consent"});
}

//------------------------------------------------------------------------------
// canvas for pressCircle 
//------------------------------------------------------------------------------

// a few globals for pressCircle
var canvasPressCircle    = document.getElementById('pressCanvas');
canvasPressCircle.width  = 400;
canvasPressCircle.height = 400;
var contextPressCircle = canvasPressCircle.getContext('2d');

// draw press circle and ring
function updatePressCircle(circleRadius, ringRadius) {
  // clear canvas
  contextPressCircle.clearRect(0, 0, 400, 400);

  // draw solid circle
  contextPressCircle.beginPath();
  contextPressCircle.arc(200, 200, circleRadius, 0, 2 * Math.PI);
  contextPressCircle.fillStyle = '#D989BC';
  contextPressCircle.fill();

  // draw ring
  contextPressCircle.beginPath();
  contextPressCircle.lineWidth = 5;
  contextPressCircle.arc(200, 200, ringRadius, 0, 2 * Math.PI);
  contextPressCircle.stroke();   
}

//------------------------------------------------------------------------------
// canvas for slider 
//------------------------------------------------------------------------------

// a few globals for pressCircle
var canvasSlider    = document.getElementById('sliderCanvas');
canvasSlider.width  = 750;
canvasSlider.height = 400;
var contextSlider = canvasSlider.getContext('2d');

// draw press circle and ring
function updateSlider() {
  // clear canvas
  contextSlider.clearRect(0, 0, 750, 400);

  // draw line
  contextSlider.beginPath();
  contextSlider.moveTo(30, 200);
  contextSlider.lineTo(750, 200);
  contextSlider.lineWidth = 5;
  contextSlider.stroke();

  // draw user box
  contextSlider.beginPath();
  contextSlider.rect(user_x, 180, 20, 40);
  contextSlider.fillStyle = '#D989BC';
  contextSlider.fill();
  
  // draw box
  contextSlider.globalAlpha = 0.5;
  contextSlider.beginPath();
  contextSlider.rect(box_x, 170, box_size, 60);
  contextSlider.fillStyle = '#F3B73B';
  contextSlider.fill();
  contextSlider.globalAlpha = 1.0;

  // '#F3B73B'
  //contextPressCircle.fillStyle = '#D989BC';
  //contextPressCircle.fill();

  window.requestAnimationFrame(updateSlider);
}