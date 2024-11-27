package utilhttp

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"
	"utils/static"
)

const (
	HTTP_REQUEST_TIMEOUT = 30 * time.Second
)

func HttpGetRequest[T any](baseUrl *url.URL, params url.Values, token string) (T, error) {
	var result T

	baseUrl.RawQuery = params.Encode()
	req, err := initRequest(http.MethodGet, baseUrl.String(), token, nil)
	if err != nil {
		return result, fmt.Errorf("failed to initialize request: %w", err)
	}

	client := http.Client{
		Timeout: HTTP_REQUEST_TIMEOUT,
	}
	resp, err := client.Do(req)
	if err != nil {
		return result, fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return result, fmt.Errorf("request failed with status %s: %s %s", resp.Status, string(body), baseUrl.String())
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return result, fmt.Errorf("failed to decode response: %w", err)
	}

	return result, nil
}

func HttpPostRequest[T any](url *url.URL, body []byte, token string) (T, error) {
	var result T

	req, err := initRequest(http.MethodPost, url.String(), token, bytes.NewBuffer(body))
	if err != nil {
		return result, fmt.Errorf("failed to initialize request: %w", err)
	}

	client := http.Client{
		Timeout: HTTP_REQUEST_TIMEOUT,
	}
	resp, err := client.Do(req)
	if err != nil {
		return result, fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		respBody, _ := io.ReadAll(resp.Body)
		return result, fmt.Errorf("request failed with status %s: %s", resp.Status, string(respBody))
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return result, fmt.Errorf("failed to decode response: %w", err)
	}

	return result, nil
}

func HttpPutRequest(url *url.URL, token string) error {
	req, err := initRequest(http.MethodPut, url.String(), token, nil)
	if err != nil {
		return fmt.Errorf("failed to initialize request: %w", err)
	}

	client := http.Client{
		Timeout: HTTP_REQUEST_TIMEOUT,
	}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusNoContent {
		respBody, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("request failed with status %s: %s", resp.Status, string(respBody))
	}

	return nil
}

func HttpDeleteRequest(url *url.URL, token string) error {
	req, err := initRequest(http.MethodDelete, url.String(), token, nil)
	if err != nil {
		return fmt.Errorf("failed to initialize request: %w", err)
	}

	client := http.Client{
		Timeout: HTTP_REQUEST_TIMEOUT,
	}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusNoContent {
		respBody, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("request failed with status %s: %s", resp.Status, string(respBody))
	}

	return nil
}

func initRequest(method, url, token string, body io.Reader) (*http.Request, error) {
	req, err := http.NewRequest(method, url, body)
	if err != nil {
		return nil, err
	}
	req.Header.Set(static.CONTENT_TYPE, static.APPLICATION_JSON)
	req.Header.Set(static.AUTHORIZATION, fmt.Sprintf("%s %s", static.BEARER, token))

	return req, nil
}
