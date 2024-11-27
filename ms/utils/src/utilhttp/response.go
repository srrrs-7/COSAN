package utilhttp

import (
	"encoding/json"
	"fmt"
	"net/http"
)

// JsonToStruct unmarshals JSON bytes into a struct.
func JsonToStruct[T any](j string, v *T) error {
	if err := json.Unmarshal([]byte(j), v); err != nil {
		return fmt.Errorf("failed to unmarshal JSON: %w", err)
	}
	return nil
}

func ResponseOk[T any](w http.ResponseWriter, msg T) {
	writeResponse(w, http.StatusOK, msg)
}

func ResponseInternalServerError[T any](w http.ResponseWriter, msg T) {
	writeResponse(w, http.StatusInternalServerError, msg)
}

func ResponseBadRequest[T any](w http.ResponseWriter, msg T) {
	writeResponse(w, http.StatusBadRequest, msg)
}

func ResponseNotFound[T any](w http.ResponseWriter, msg T) {
	writeResponse(w, http.StatusNotFound, msg) // Corrected status code
}

// writeResponse is a helper function to write the response with the given status code and message.
func writeResponse[T any](w http.ResponseWriter, statusCode int, msg T) {
	res, err := json.Marshal(msg)
	if err != nil {
		panic(err.Error())
	}
	w.Header().Set("Content-Type", "application/json") // Use direct string
	w.WriteHeader(statusCode)
	w.Write([]byte(res))
}
