package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
)

type FlinkJobSubmission struct {
	EntryClass    string `json:"entryClass"`
	ProgramArgs   string `json:"programArgs"`
	Parallelism   int    `json:"parallelism"`
	JobName       string `json:"jobName"`
}

func uploadJar(jarPath string) (string, error) {
	file, err := os.Open(jarPath)
	if err != nil {
		return "", err
	}
	defer file.Close()

	response, err := http.Post("http://localhost:8081/jars/upload", "application/x-java-archive", file)
	if err != nil {
		return "", err
	}
	defer response.Body.Close()

	body, err := ioutil.ReadAll(response.Body)
	if err != nil {
		return "", err
	}

	var result map[string]interface{}
	json.Unmarshal(body, &result)
	jarID := result["filename"].(string)

	return jarID, nil
}

func submitFlinkJob(jarID, entryClass, jobName string) error {
	submission := FlinkJobSubmission{
		EntryClass:  entryClass,
		ProgramArgs: "",
		Parallelism: 1,
		JobName:     jobName,
	}

	data, err := json.Marshal(submission)
	if err != nil {
		return err
	}

	response, err := http.Post(fmt.Sprintf("http://localhost:8081/jars/%s/run", jarID), "application/json", bytes.NewBuffer(data))
	if err != nil {
		return err
	}
	defer response.Body.Close()

	if response.StatusCode != http.StatusOK {
		body, _ := ioutil.ReadAll(response.Body)
		return fmt.Errorf("failed to submit Flink job: %s", string(body))
	}

	fmt.Println("Flink job submitted successfully!")
	return nil
}

func main() {
	jarID, err := uploadJar("path/to/your/flink-job.jar")
	if err != nil {
		fmt.Printf("Error uploading jar: %s\n", err)
		return
	}

	err = submitFlinkJob(jarID, "com.neuraserver.flink.DynamicDataProcessor", "Dynamic Data Processor")
	if err != nil {
		fmt.Printf("Error submitting job: %s\n", err)
	}
}
