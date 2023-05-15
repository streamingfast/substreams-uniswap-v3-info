package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"path/filepath"
	"regexp"
	"strings"

	_ "github.com/streamingfast/kvdb/store/badger3"
	"google.golang.org/protobuf/proto"

	"github.com/spf13/cobra"
	"github.com/streamingfast/cli"
	. "github.com/streamingfast/cli"
	"github.com/streamingfast/kvdb/store"
	"github.com/streamingfast/logging"
	pbinfo "github.com/streamingfast/substreams-uniswap-v3-info/pb/sf/substreams/uniswap/v3"
	"go.uber.org/zap"
)

var zlog, tracer = logging.RootLogger("project", "github.com/acme/project")

func main() {
	logging.InstantiateLoggers()

	Run(
		"inject-fake-kv <data-dir>",
		"Inject fake data into the KV (from real data files)",
		Execute(run),
		MinimumNArgs(1),
	)
}

var rePoolDayDatasFilename = regexp.MustCompile(`pool_day_datas_(0x[0-9a-fA-F]+).json`)

func run(cmd *cobra.Command, args []string) error {
	dataDir := args[0]

	zlog.Info("executing", zap.String("data_dir", dataDir))

	kvStore, err := store.New(fmt.Sprintf("badger3://%s/pool_day_datas.db", cli.AbsolutePath(dataDir)))
	cli.NoError(err, "new kvdb store")

	cli.NoError(filepath.WalkDir(dataDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if d.IsDir() {
			return nil
		}

		if matches := rePoolDayDatasFilename.FindAllStringSubmatch(d.Name(), 1); len(matches) == 1 {
			address := matches[0][1]
			fmt.Printf("Inserting pool day datas (Address %s)\n", address)
			insertPool(cmd.Context(), kvStore, path, address)
		}

		return nil
	}), "Unable to walk for pool day datas")

	cli.NoError(kvStore.FlushPuts(cmd.Context()), "flushing puts")

	fmt.Println("completed")
	return nil
}

type GraphPoolDayData struct {
	Date      uint64 `json:"date"`
	VolumeUSD string `json:"volumeUSD"`
	TVLUSD    string `json:"tvlUSD"`
	FeesUSD   string `json:"feesUSD"`
	Pool      struct {
		FeeTier string `json:"feeTier"`
	}
}

func (d *GraphPoolDayData) Key(address string) string {
	return fmt.Sprintf("pool:day:%s:%d", strings.ToLower(address), d.Date)
}

func (d *GraphPoolDayData) ToProto() *pbinfo.PoolDayData {
	return &pbinfo.PoolDayData{
		Date:      d.Date,
		VolumeUsd: d.VolumeUSD,
		TvlUsd:    d.TVLUSD,
		FeesUsd:   d.FeesUSD,
		Pool: &pbinfo.Pool{
			FeeTier: d.Pool.FeeTier,
		},
	}
}

func insertPool(ctx context.Context, kvStore store.KVStore, path string, address string) {
	type document struct {
		Data struct {
			PoolDayDatas []*GraphPoolDayData
		}
	}

	doc := document{}
	cli.NoError(json.Unmarshal([]byte(cli.ReadFile(path)), &doc), "unable to read pool day datas file")

	for _, poolDayData := range doc.Data.PoolDayDatas {
		bytes, err := proto.Marshal(poolDayData.ToProto())
		cli.NoError(err, "unable to marshal pool day data")

		cli.NoError(kvStore.Put(ctx, []byte(poolDayData.Key(address)), bytes), "put key")
	}
}
