var tracksArr = []

window.onbeforeunload = function(e) {
    return 'Want to stop the music ?';
};

function refreshTracks() {
	$.ajax({
	    url   : "/refresh",
	    type  : "get",
	    dataType: "html",
	    success : function(result){
	    	loadTracks();
	    }
	});
}

function loadTracks() {
	$.ajax({
	    url   : "/music",
	    type  : "get",
	    dataType: "json",
	    beforeSend  : function(){
	    	
	    },
	    success : function(result){
	    	var track_list = document.getElementById('track-list')
	    	,	tracks = ""
	    	
	    	// reset empty
	    	track_list.innerHTML = ""

	    	result.forEach(function(e) {
	    		tracksArr.push(e)
	    		tracks += `	<li class="list_item" onclick="play('${e}')">
		                        <div class="thumb"> </div>
		                        <div class="info">
		                            <div class="title">${e}</div>
		                        </div>
	                      	</li>`
	    	})
	    	track_list.insertAdjacentHTML('beforeend', tracks);
	    }
	});
}

// call loadTracks() immediately
loadTracks();

function play(fname) {

	$("div > div.title.e").html(fname)

	var audio = document.getElementsByTagName("audio")[0]
	
	$("audio > source").attr("src", `/assets/music/${fname}`)
	$("audio > source").attr("file", fname)

  	audio.load(); //call this to just preload the audio without playing
  	audio.play();
}

function nextMusic() {
	var currentItem = $("audio > source").attr("file")
	var currentIndex = tracksArr.indexOf(currentItem);
	var nextIndex = (currentIndex + 1) % tracksArr.length;
	play(tracksArr[nextIndex])
}